use std::{ops::Mul, str::FromStr};
use log::info;
use serde::{Serialize, Deserialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey};
use spl_token_lending::state::{Reserve, SLOTS_PER_YEAR};
use spl_token_lending::math::{Decimal, TryAdd, TryMul, TryDiv};

use crate::{AssetSymbol, PRODUCTION_CONFIG_JSON, utils::ProgramConfig, Stats};

const MNDE_RATE: f64 = 0.14269371512;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APY {
    pub asset: AssetSymbol,
    pub name: String,
    pub price: f64,
    pub supply: f64,
    pub borrow: f64,
    pub supply_rewards: f64,
    pub borrow_rewards: f64,
    pub weight_supply: u8,
    pub weight_borrow: u8,
    pub mnde_supply_rewards: Option<f64>,
}

impl APY {
    pub fn from_assets(rpc_client: &RpcClient, assets: &Vec<AssetSymbol>) -> Vec<Self> {
        let program_config : ProgramConfig = serde_json::from_str(PRODUCTION_CONFIG_JSON).unwrap();
        // Get all production PubKeys
        let mut account_pks = Vec::<Pubkey>::new();
        for &asset_symbol in assets {
            let reserve_json= program_config.markets[0].reserves.iter().find(|e| e.asset == asset_symbol).unwrap();
            let reserve_pk = Pubkey::from_str(&reserve_json.address.to_string()).unwrap();
            account_pks.push(reserve_pk);
        }

        let reward_stats : serde_json::Value = reqwest::blocking::get("https://api.solend.fi/liquidity-mining/reward-stats").unwrap().json().unwrap();
        let accounts = rpc_client.get_multiple_accounts(&account_pks).unwrap();
        let mut result = Vec::<APY>::new();
        for (index, account) in accounts.iter().enumerate() {
            let data = account.as_ref().unwrap().data.clone();
            let reserve = Reserve::unpack_from_slice(&data).unwrap();
            let asset_symbol = assets[index];
            result.push(Self::from_reserve(rpc_client, &reserve, asset_symbol, &reward_stats));
        }
        return result;
    }

    pub fn from_asset(rpc_client: &RpcClient, asset_symbol: AssetSymbol) -> Self {
        let program_config : ProgramConfig = serde_json::from_str(PRODUCTION_CONFIG_JSON).unwrap();
        let reserve_json= program_config.markets[0].reserves.iter().find(|e| e.asset == asset_symbol).unwrap();
        let reserve_pk = Pubkey::from_str(&reserve_json.address.to_string()).unwrap();
        let account_data = rpc_client.get_account_data(&reserve_pk).unwrap();
        let reserve = Reserve::unpack_from_slice(&account_data).unwrap();
        let reward_stats : serde_json::Value = reqwest::blocking::get("https://api.solend.fi/liquidity-mining/reward-stats").unwrap().json().unwrap();
        
        return Self::from_reserve(rpc_client, &reserve, asset_symbol, &reward_stats);
    }

    fn from_reserve(rpc_client: &RpcClient, reserve: &Reserve, asset_symbol: AssetSymbol, reward_stats: &serde_json::Value) -> Self {
        info!("Calculate {} APY", asset_symbol);
        let market_price = (reserve.liquidity.market_price.to_scaled_val().unwrap() as f64) / 1_000_000_000_000_000_000f64;
        let supply_apy = Self::calculate_supply(&reserve);
        let borrow_apy = Self::calculate_borrow(&reserve);
        let rewards = Self::calculate_annual_tokens(rpc_client, &reserve, asset_symbol, &reward_stats);
        let mnde_supply_rewards = if let Some(value) = rewards.4 { value } else { 0f64 }; 

        return Self {
            asset: asset_symbol,
            name: asset_symbol.name(),
            price: market_price,
            supply: supply_apy + rewards.0 + mnde_supply_rewards,
            borrow: borrow_apy - rewards.1,
            supply_rewards: rewards.0,
            borrow_rewards: rewards.1,
            weight_supply: rewards.2,
            weight_borrow: rewards.3,
            mnde_supply_rewards: rewards.4,
        };
    }

    fn calculate_borrow(reserve: &Reserve) -> f64 {
        let current_utilization = Self::calculate_utilization_ratio(reserve);
        let optimal_utilization = reserve.config.optimal_utilization_rate as f64 / 100f64;
        let borrow_apy = {
            if optimal_utilization == 1f64 || current_utilization < optimal_utilization {
                let normalized_factor = current_utilization / optimal_utilization;
                let optimal_borrow_rate = reserve.config.optimal_borrow_rate as f64 / 100f64;
                let min_borrow_rate = reserve.config.min_borrow_rate as f64 / 100f64;
                normalized_factor * (optimal_borrow_rate - min_borrow_rate) + min_borrow_rate
            } else {
                let normalized_factor = (current_utilization - optimal_utilization) / (1f64 - optimal_utilization);
                let optimal_borrow_rate = reserve.config.optimal_borrow_rate as f64 / 100f64;
                let max_borrow_rate = reserve.config.max_borrow_rate as f64 / 100f64;
                normalized_factor * (max_borrow_rate - optimal_borrow_rate) + optimal_borrow_rate
            }
        };
        return borrow_apy;
    }

    fn calculate_supply(reserve: &Reserve) -> f64 {
        let current_utilization = Self::calculate_utilization_ratio(reserve);
        let borrow_apy = Self::calculate_borrow(reserve);
        let supply_apy = current_utilization * borrow_apy;
        return supply_apy;
    }

    fn calculate_utilization_ratio(reserve: &Reserve) -> f64 {
        let borrowed_ammount = reserve.liquidity.borrowed_amount_wads.try_round_u64().unwrap();
        let available_ammount = reserve.liquidity.available_amount;
        let current_utilization = borrowed_ammount as f64 / (available_ammount + borrowed_ammount) as f64;
        return current_utilization;
    }

    fn calculate_annual_tokens(rpc_client: &RpcClient, reserve: &Reserve, asset_symbol: AssetSymbol, reward_stats: &serde_json::Value) -> (f64, f64, u8, u8, Option<f64>) {
        let program_config : ProgramConfig = serde_json::from_str(PRODUCTION_CONFIG_JSON).unwrap();

        let mint_address = &program_config.assets.iter().find(|a| a.symbol == asset_symbol).unwrap().mint_address;
        let reward_stats = &reward_stats[mint_address];

        let supply_reward_rate = &reward_stats["supply"]["rewardRates"].as_array();
        let supply_reward_rate: (Decimal, u8) = if let Some(reward_rate) = supply_reward_rate {
            let last_reward_rate= reward_rate.iter().last().unwrap();
            let reward_rate_string = &last_reward_rate["rewardRate"].as_str().unwrap_or("0");
            let result = string_to_decimal(reward_rate_string.to_string());
            let name = &last_reward_rate["name"].as_str().unwrap_or("0");
            let name = if name.is_empty() { "0" } else { name };
            let name = &name[..1];
            (result, name.parse().unwrap_or(0))
        } else { (Decimal::zero(), 0) };

        let borrow_reward_rate = &reward_stats["borrow"]["rewardRates"].as_array();
        let borrow_reward_rate: (Decimal, u8) = if let Some(reward_rate) = borrow_reward_rate {
            let last_reward_rate= reward_rate.iter().last().unwrap();
            let reward_rate_string = &last_reward_rate["rewardRate"].as_str().unwrap_or("0");
            let result = string_to_decimal(reward_rate_string.to_string());
            let name = &last_reward_rate["name"].as_str().unwrap_or("0");
            let name = if name.is_empty() { "0" } else { name };
            let name = &name[..1];
            (result, name.parse().unwrap_or(0))
        } else { (Decimal::zero(), 0) };

        if supply_reward_rate.1 != 0 || borrow_reward_rate.1 != 0 {
            let market_price = (reserve.liquidity.market_price.to_scaled_val().unwrap() as f64) / 1_000_000_000_000_000_000f64;
            let available_ammount = (reserve.liquidity.available_amount as f64).mul(market_price);
            let borrowed_ammount = (reserve.liquidity.borrowed_amount_wads.try_round_u64().unwrap() as f64).mul(market_price);
            let total_supply = available_ammount + borrowed_ammount;
            let mint_decimals = reserve.liquidity.mint_decimals.into();

            // TODO: Clean calculations
            let slnd_price = Stats::get_slnd_price(rpc_client);

            let mut mnde_supply_reward_apy : Option<f64> = None;
            if asset_symbol == AssetSymbol::mSOL { 
                let mnde_price= Stats::get_mnde_price(rpc_client);
                let mnde_supply_reward_per_dollar = MNDE_RATE / (total_supply as f64) * 10_f64.powi(mint_decimals);
                let mnde_supply_reward = mnde_supply_reward_per_dollar * SLOTS_PER_YEAR as f64;
                mnde_supply_reward_apy = Some(mnde_supply_reward * mnde_price);
            }

            let supply_reward_apy = supply_reward_rate.0.try_div(10_u64.pow(18)).unwrap().to_scaled_val().unwrap() as f64;
            let supply_reward_apy = supply_reward_apy * slnd_price / (total_supply as f64) * 10_f64.powi(mint_decimals);
            let borrow_reward_apy = borrow_reward_rate.0.try_div(10_u64.pow(18)).unwrap().to_scaled_val().unwrap() as f64;
            let borrow_reward_apy = borrow_reward_apy * slnd_price / (borrowed_ammount as f64) * 10_f64.powi(mint_decimals);

            return (supply_reward_apy, borrow_reward_apy, supply_reward_rate.1, borrow_reward_rate.1, mnde_supply_reward_apy);
        } 

        return (0f64, 0f64, 0, 0, None);
    }
}

fn string_to_decimal(number: String) -> Decimal {
    let mut result= Decimal::zero();
    for c in number.chars() {
        let digit = c.to_string().parse::<u64>().unwrap();
        let decimal= Decimal::from(digit).try_div(Decimal::from(10_u64.pow(18))).unwrap();
        result = result.try_mul(10).unwrap().try_add(decimal).unwrap();
    }
    result.try_div(10_u64.pow(18)).unwrap()
}

use std::{ops::Mul, str::FromStr};
use log::info;
use serde::{Serialize, Deserialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey};
use spl_token_lending::state::Reserve;
use spl_token_lending::math::{Decimal, TryDiv};

use crate::utils::{TokenRewardStat, Reward};
use crate::{AssetSymbol, PRODUCTION_CONFIG_JSON, utils::ProgramConfig, Stats};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APY {
    pub asset: AssetSymbol,
    pub name: String,
    pub price: f64,
    pub supply: f64,
    pub borrow: f64,
    pub supply_rewards: f64,
    pub borrow_rewards: f64,
    pub weight_supply: String,
    pub weight_borrow: String,
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
        let external_reward_stats : serde_json::Value = reqwest::blocking::get("https://api.solend.fi/liquidity-mining/external-reward-stats").unwrap().json().unwrap();
        let slnd_price = Stats::get_slnd_price(rpc_client);
        let mnde_price = Stats::get_mnde_price(rpc_client);

        let accounts = rpc_client.get_multiple_accounts(&account_pks).unwrap();
        let mut result = Vec::<APY>::new();
        for (index, account) in accounts.iter().enumerate() {
            let data = account.as_ref().unwrap().data.clone();
            let reserve = Reserve::unpack_from_slice(&data).unwrap();
            let asset_symbol = assets[index];
            result.push(Self::from_reserve(&reserve, asset_symbol, &reward_stats, &external_reward_stats, slnd_price, mnde_price));
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
        let external_reward_stats : serde_json::Value = reqwest::blocking::get("https://api.solend.fi/liquidity-mining/external-reward-stats").unwrap().json().unwrap();
        let slnd_price = Stats::get_slnd_price(rpc_client);
        let mnde_price = Stats::get_slnd_price(rpc_client);
        
        return Self::from_reserve(&reserve, asset_symbol, &reward_stats, &external_reward_stats, slnd_price, mnde_price);
    }

    fn from_reserve(reserve: &Reserve, asset_symbol: AssetSymbol, reward_stats: &serde_json::Value, external_reward_stats: &serde_json::Value, slnd_price: f64, mnde_price: f64) -> Self {
        info!("Calculate {} APY", asset_symbol);
        let market_price = (reserve.liquidity.market_price.to_scaled_val().unwrap() as f64) / 1_000_000_000_000_000_000f64;
        let supply_apy = Self::calculate_supply(&reserve);
        let borrow_apy = Self::calculate_borrow(&reserve);
        let rewards = Self::calculate_annual_tokens(&reserve, asset_symbol, &reward_stats, &external_reward_stats, slnd_price, mnde_price);
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

    fn calculate_annual_tokens(reserve: &Reserve, asset_symbol: AssetSymbol, reward_stats: &serde_json::Value, external_reward_stats: &serde_json::Value, slnd_price: f64, mnde_price: f64) -> (f64, f64, String, String, Option<f64>) {
        let program_config : ProgramConfig = serde_json::from_str(PRODUCTION_CONFIG_JSON).unwrap();

        let mint_address = &program_config.assets.iter().find(|a| a.symbol == asset_symbol).unwrap().mint_address;

        // Reward Rates
        let token_reward_stats = &reward_stats[mint_address];
        // TODO: Move this to ::from_value()
        let token_reward_stats = TokenRewardStat {
            supply: serde_json::from_value(token_reward_stats["supply"].clone()).unwrap(), 
            borrow: serde_json::from_value(token_reward_stats["borrow"].clone()).unwrap(), 
        };

        let supply_reward: (Decimal, String) = get_reward_rate_and_name(token_reward_stats.supply);
        let borrow_reward: (Decimal, String) = get_reward_rate_and_name(token_reward_stats.borrow);

        // External Reward Rates
        let token_external_reward_stats = &external_reward_stats[mint_address];
        let token_external_reward_stats = TokenRewardStat {
            supply: serde_json::from_value(token_external_reward_stats["supply"].clone()).unwrap(), 
            borrow: serde_json::from_value(token_external_reward_stats["borrow"].clone()).unwrap(), 
        };

        let supply_external_reward: (Decimal, String) = get_reward_rate_and_name(token_external_reward_stats.supply);
        let borrow_external_reward: (Decimal, String) = get_reward_rate_and_name(token_external_reward_stats.borrow);

        if !supply_reward.1.is_empty() || !borrow_reward.1.is_empty() {
            let market_price = (reserve.liquidity.market_price.to_scaled_val().unwrap() as f64) / 1_000_000_000_000_000_000f64;
            let available_ammount = (reserve.liquidity.available_amount as f64).mul(market_price);
            let borrowed_ammount = (reserve.liquidity.borrowed_amount_wads.try_round_u64().unwrap() as f64).mul(market_price);
            let total_supply = available_ammount + borrowed_ammount;
            let mint_decimals = reserve.liquidity.mint_decimals.into();

            let mut supply_external_reward_apy : f64 = 0.0;
            if supply_external_reward.0 != Decimal::zero() || borrow_external_reward.0 != Decimal::zero() {
                supply_external_reward_apy = supply_external_reward.0.try_div(10_u64.pow(18)).unwrap().to_scaled_val().unwrap() as f64;
                supply_external_reward_apy = supply_external_reward_apy * mnde_price / (total_supply as f64) * 10_f64.powi(mint_decimals);
            }
            let supply_external_reward_apy : Option<f64> = if supply_external_reward_apy == 0.0 { None } else { Some(supply_external_reward_apy) };

            /* Borrow Rewards not available 
            let mut borrow_external_reward_apy : f64 = 0.0;
            if borrow_external_reward.0 != Decimal::zero() || borrow_external_reward.0 != Decimal::zero() {
                borrow_external_reward_apy = borrow_external_reward.0.try_div(10_u64.pow(18)).unwrap().to_scaled_val().unwrap() as f64;
                borrow_external_reward_apy = borrow_external_reward_apy * mnde_price / (borrowed_ammount as f64) * 10_f64.powi(mint_decimals);
            }
            let borrow_external_reward_apy : Option<f64> = if borrow_external_reward_apy == 0.0 { None } else { Some(borrow_external_reward_apy) };
            */

            let supply_reward_apy = supply_reward.0.try_div(10_u64.pow(18)).unwrap().to_scaled_val().unwrap() as f64;
            let supply_reward_apy = supply_reward_apy * slnd_price / (total_supply as f64) * 10_f64.powi(mint_decimals);
            let borrow_reward_apy = borrow_reward.0.try_div(10_u64.pow(18)).unwrap().to_scaled_val().unwrap() as f64;
            let borrow_reward_apy = borrow_reward_apy * slnd_price / (borrowed_ammount as f64) * 10_f64.powi(mint_decimals);

            return (supply_reward_apy, borrow_reward_apy, supply_reward.1, borrow_reward.1, supply_external_reward_apy);
        } 

        return (0f64, 0f64, String::new(), String::new(), None);
    }
}

fn get_reward_rate_and_name(token_reward_stats: Option<Reward>) -> (Decimal, String) {
    match token_reward_stats {
        Some(reward) => {
            match reward.reward_rates {
                Some(reward_rate) => {
                    let last_reward_rate = reward_rate.last().unwrap();
                    let name = last_reward_rate.name.clone().unwrap_or_default();
                    (last_reward_rate.reward_rate, name)
                },
                None => (Decimal::zero(), String::new())
            }
        },
        None => (Decimal::zero(), String::new())
    }
}
use std::{ops::Mul, str::FromStr};
use log::info;
use serde::{Serialize, Deserialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey};
use spl_token_lending::state::{Reserve, SLOTS_PER_YEAR};

use crate::{AssetSymbol, PRODUCTION_CONFIG_JSON, utils::ProgramConfig};

const SLND_RATE: f64 = 0.1585;
const TOTAL_WEIGHT: u8 = 11;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APY {
    pub asset: AssetSymbol,
    pub price: f64,
    pub supply: f64,
    pub borrow: f64,
    pub supply_rewards: f64,
    pub borrow_rewards: f64,
    pub weight: u8,
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

        let accounts = rpc_client.get_multiple_accounts(&account_pks).unwrap();
        let mut result = Vec::<APY>::new();
        for (index, account) in accounts.iter().enumerate() {
            let data = account.as_ref().unwrap().data.clone();
            let reserve = Reserve::unpack_from_slice(&data).unwrap();
            let asset_symbol = assets[index];
            result.push(Self::from_reserve(&reserve, asset_symbol));
        }
        return result;
    }

    pub fn from_asset(rpc_client: &RpcClient, asset_symbol: AssetSymbol) -> Self {
        let program_config : ProgramConfig = serde_json::from_str(PRODUCTION_CONFIG_JSON).unwrap();
        let reserve_json= program_config.markets[0].reserves.iter().find(|e| e.asset == asset_symbol).unwrap();
        let reserve_pk = Pubkey::from_str(&reserve_json.address.to_string()).unwrap();
        let account_data = rpc_client.get_account_data(&reserve_pk).unwrap();
        let reserve = Reserve::unpack_from_slice(&account_data).unwrap();
        
        return Self::from_reserve(&reserve, asset_symbol);
    }

    fn from_reserve(reserve: &Reserve, asset_symbol: AssetSymbol) -> Self {
        info!("Calculate {} APY", asset_symbol);
        let market_price = (reserve.liquidity.market_price.to_scaled_val().unwrap() as f64) / 1_000_000_000_000_000_000f64;
        let supply_apy = Self::calculate_supply(&reserve);
        let borrow_apy = Self::calculate_borrow(&reserve);
        let rewards = Self::calculate_annual_tokens(&reserve, asset_symbol);
        
        return Self {
            asset: asset_symbol,
            price: market_price,
            supply: supply_apy,
            borrow: borrow_apy,
            supply_rewards: rewards.0,
            borrow_rewards: rewards.1,
            weight: rewards.2,
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

    fn calculate_annual_tokens(reserve: &Reserve, asset_symbol: AssetSymbol) -> (f64, f64, u8) {
        let program_config : ProgramConfig = serde_json::from_str(PRODUCTION_CONFIG_JSON).unwrap();
        let reserve_json = program_config.markets[0].reserves.iter().find(|e| e.asset == asset_symbol).unwrap();
        let weight = if let Some(weight) = reserve_json.weight { weight } else { 0 };

        if weight != 0 {
            let market_price = (reserve.liquidity.market_price.to_scaled_val().unwrap() as f64) / 1_000_000_000_000_000_000f64;
            let available_ammount = (reserve.liquidity.available_amount as f64).mul(market_price);
            let borrowed_ammount = (reserve.liquidity.borrowed_amount_wads.try_round_u64().unwrap() as f64).mul(market_price);
            let total_supply = available_ammount + borrowed_ammount;
            let mint_decimals = reserve.liquidity.mint_decimals.into();

            // TODO: Clean calculations
            let reward_split = weight as f64 / TOTAL_WEIGHT as f64;
            let supply_reward_per_thousand = SLND_RATE * (reward_split / 2.0) / (total_supply as f64 / 1000.0) * 10_f64.powi(mint_decimals); 
            let borrow_reward_per_thousand = SLND_RATE * (reward_split / 2.0) / (borrowed_ammount as f64 / 1000.0) * 10_f64.powi(mint_decimals); 
            let supply_reward = supply_reward_per_thousand * SLOTS_PER_YEAR as f64;
            let borrow_reward= borrow_reward_per_thousand * SLOTS_PER_YEAR as f64;

            return (supply_reward, borrow_reward, weight);
        } 

        return (0f64, 0f64, 0);
    }
}

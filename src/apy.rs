use std::str::FromStr;
use std::fmt;
use chrono::offset::Utc;
use chrono::DateTime;
use log::info;
use serde::{Serialize, Deserialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey};
use spl_token_lending::state::Reserve;

use crate::{AssetQuote, PRODUCTION_CONFIG_JSON, utils::ProgramConfig};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APY {
    pub asset: AssetQuote,
    pub price: f64,
    pub supply: f64,
    pub borrow: f64,
}
impl APY {
    pub fn from_asset(rpc_client: &RpcClient, asset_quote: AssetQuote) -> Self {
        info!("Calculate {} APY", asset_quote);
        let program_config : ProgramConfig = serde_json::from_str(PRODUCTION_CONFIG_JSON).unwrap();
        let reserve_json= program_config.markets[0].reserves.iter().find(|e| e.asset == asset_quote).unwrap();
        let reserve_pk = Pubkey::from_str(&reserve_json.address.to_string()).unwrap();
        let account_data = rpc_client.get_account_data(&reserve_pk).unwrap();
        let reserve = Reserve::unpack_from_slice(&account_data).unwrap();
        
        let market_price = (reserve.liquidity.market_price.to_scaled_val().unwrap() as f64) / (1_000_000_000_000_000_000f64);
        let supply_apy = Self::calculate_supply(&reserve);
        let borrow_apy = Self::calculate_borrow(&reserve);

        return Self {
            asset: asset_quote,
            price: market_price,
            supply: supply_apy,
            borrow: borrow_apy,
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APYDataPoint {
    pub date: DateTime<Utc>,
    pub data_type: DataType,
    pub apys: Vec<APY>,
}


#[derive(Serialize, Deserialize, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum DataType {
    MINUTE,
    HOUR,
    DAY,
    WEEK,
}

impl FromStr for DataType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MINUTE"| "minute" => Ok(DataType::MINUTE),
            "HOUR"| "hour" => Ok(DataType::HOUR),
            "DAY" | "day" => Ok(DataType::DAY),
            "WEEK" | "week" => Ok(DataType::WEEK),
            _ => Err(format!("'{}' is not a valid value for DataType", s)),
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
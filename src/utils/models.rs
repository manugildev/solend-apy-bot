use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::PRODUCTION_CONFIG_JSON;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct ProgramConfig {
    pub assets: Vec<Asset>,
    pub markets: Vec<Market>,
    pub oracles: Oracle,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Asset {
    pub name: String,
    pub symbol: AssetSymbol,
    pub decimals: u8,
    pub mint_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Market {
    pub name: String,
    pub address: String,
    pub authority_address: String,
    pub transfer_authority_address: String,
    pub reserves: Vec<Reserve>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Reserve {
    pub asset: AssetSymbol,
    pub address: String,
    pub collateral_mint_address: String,
    pub collateral_supply_address: String,
    pub liquidity_address: String,
    pub liquidity_fee_receiver_address: String,
    pub user_supply_cap: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Oracle {
    #[serde(rename = "pythProgramID")]
    pub pyth_program_id: String,
    #[serde(rename = "switchboardProgramID")]
    pub switchboard_program_id: String,
    pub assets: Vec<OracleAsset>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct OracleAsset {
    pub asset: AssetSymbol,
    pub oracle_address: String,
    pub price_address: String,
    pub switchboard_feed_address: String,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum AssetSymbol {
    // TODO: Auto generate this form PRODUCTION_CONFIG_JSON
    SOL,
    USDC,
    USDT,
    ETH,
    BTC,
    SRM,
    FTT,
    RAY,
    MER,
    SBR,
    MNGO,
    mSOL,
    PAI,
    UST,
}

#[allow(dead_code)]
impl AssetSymbol {
    pub fn name(&self) -> String {
        let assets = serde_json::from_str::<ProgramConfig>(PRODUCTION_CONFIG_JSON).unwrap().assets;
        return match self {
            _ => assets.iter().find(|a| a.symbol == *self).unwrap().name.clone(),
        };
    }
}

impl FromStr for AssetSymbol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SOL" => Ok(AssetSymbol::SOL),
            "USDC" => Ok(AssetSymbol::USDC),
            "USDT" => Ok(AssetSymbol::USDT),
            "ETH" => Ok(AssetSymbol::ETH),
            "BTC" => Ok(AssetSymbol::BTC),
            "SRM" => Ok(AssetSymbol::SRM),
            "FTT" => Ok(AssetSymbol::FTT),
            "RAY" => Ok(AssetSymbol::RAY),
            "MER" => Ok(AssetSymbol::MER),
            "SBR" => Ok(AssetSymbol::SBR),
            "MNGO" => Ok(AssetSymbol::MNGO),
            "MSOL" => Ok(AssetSymbol::mSOL),
            "PAI" => Ok(AssetSymbol::PAI),
            "UST" => Ok(AssetSymbol::UST),
            _ => Err(format!("'{}' is not a valid value for AssetSymbol", s)),
        }
    }
}

impl fmt::Display for AssetSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChartData {
    pub name: AssetSymbol,
    pub data: Vec<(String, f64)>,
}

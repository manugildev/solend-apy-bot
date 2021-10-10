use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

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
    pub symbol: AssetQuote,
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
    pub asset: AssetQuote,
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
    pub asset: AssetQuote,
    pub oracle_address: String,
    pub price_address: String,
    pub switchboard_feed_address: String,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum AssetQuote {
    SOL,
    USDC,
    ETH,
    BTC,
    SRM,
    USDT,
    FTT,
    RAY,
    MER,
    SBR,
    MNGO,
    mSOL,
    PAI,
    UST,
}

impl FromStr for AssetQuote {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SOL" => Ok(AssetQuote::SOL),
            "USDC" => Ok(AssetQuote::USDC),
            "ETH" => Ok(AssetQuote::ETH),
            "BTC" => Ok(AssetQuote::BTC),
            "SRM" => Ok(AssetQuote::SRM),
            "USDT" => Ok(AssetQuote::USDT),
            "FTT" => Ok(AssetQuote::FTT),
            "RAY" => Ok(AssetQuote::RAY),
            "MER" => Ok(AssetQuote::MER),
            "SBR" => Ok(AssetQuote::SBR),
            "MNGO" => Ok(AssetQuote::MNGO),
            "MSOL" => Ok(AssetQuote::mSOL),
            "PAI" => Ok(AssetQuote::PAI),
            "UST" => Ok(AssetQuote::UST),
            _ => Err(format!("'{}' is not a valid value for AssetQuote", s)),
        }
    }
}

impl fmt::Display for AssetQuote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChartData {
    pub name: AssetQuote,
    pub data: Vec<(String, f64)>,
}


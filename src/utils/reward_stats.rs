use serde::{ Deserialize, Deserializer };
use spl_token_lending::math::*;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct TokenRewardStat {
    pub supply: Option<Reward>,
    pub borrow: Option<Reward>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Reward {
    pub rewards_per_share: String,
    pub total_balance: String,
    pub last_slot: u64,
    pub side: String,
    pub token_mint: String,
    pub reward_rates: Option<Vec<RewardRate>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct RewardRate {
    pub beginning_slot: u64,
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub reward_rate: Decimal,
    pub name: Option<String>,
}

pub fn deserialize_string_from_number<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(i64),
        Float(f64),
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => Ok(string_to_decimal(s)),
        StringOrNumber::Number(i) => Ok(Decimal::from(i as u64)),
        StringOrNumber::Float(f) => Ok(Decimal::from(f as u64)),
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
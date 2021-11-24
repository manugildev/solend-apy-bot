use log::info;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey, account_info::IntoAccountInfo};
use spl_token_lending::state::Reserve;
use std::{ops::Mul, str::FromStr};
use switchboard_program::{self, AggregatorState, RoundResult};

use crate::{utils::ProgramConfig, AssetSymbol, PRODUCTION_CONFIG_JSON};

const SLND_FEED_ACCOUNT: &'static str = "7QKyBR3zLRhoEH5UMjcG8emDD2J2CCDmkxv3qsa2Mqif";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub slnd_price: f64,
    pub mnde_price: f64,
    pub total_supplied: f64,
    pub total_borrowed: f64,
}

impl Stats {
    pub fn from_assets(rpc_client: &RpcClient, assets: &Vec<AssetSymbol>) -> Stats {
        let program_config: ProgramConfig = serde_json::from_str(PRODUCTION_CONFIG_JSON).unwrap();
        // Get all production PubKeys
        let mut account_pks = Vec::<Pubkey>::new();
        for &asset_symbol in assets {
            let reserve_json = program_config.markets[0]
                .reserves
                .iter()
                .find(|e| e.asset == asset_symbol)
                .unwrap();
            let reserve_pk = Pubkey::from_str(&reserve_json.address.to_string()).unwrap();
            account_pks.push(reserve_pk);
        }

        let accounts = rpc_client.get_multiple_accounts(&account_pks).unwrap();
        let mut total_supplied = 0f64;
        let mut total_borrowed = 0f64;
        for (_, account) in accounts.iter().enumerate() {
            let data = account.as_ref().unwrap().data.clone();
            let reserve = Reserve::unpack_from_slice(&data).unwrap();

            let market_price = (reserve.liquidity.market_price.to_scaled_val().unwrap() as f64) / 1_000_000_000_000_000_000f64;
            let mint_decimals = reserve.liquidity.mint_decimals.into();
            let available_ammount = (reserve.liquidity.available_amount as f64).mul(market_price) / 10_f64.powf(mint_decimals);
            let borrowed_ammount = (reserve.liquidity.borrowed_amount_wads.try_round_u64().unwrap() as f64).mul(market_price) / 10_f64.powf(mint_decimals);
            let supplied_ammount = available_ammount + borrowed_ammount;

            total_supplied += supplied_ammount;
            total_borrowed += borrowed_ammount;
        }

        info!("Calculate stats from asset");

        let slnd_price = Self::get_slnd_price(rpc_client);
        let mnde_price = Self::get_mnde_price(rpc_client);
        return Stats {
            slnd_price,
            mnde_price,
            total_supplied,
            total_borrowed,
        };
    }

    pub fn get_slnd_price(rpc_client: &RpcClient) -> f64 {
        info!("Get slnd price");
        let slnd_feed_data_pk = Pubkey::from_str(SLND_FEED_ACCOUNT).unwrap();
        let slnd_feed_data_account = rpc_client.get_account(&slnd_feed_data_pk).unwrap();
        let mut account = (slnd_feed_data_pk, slnd_feed_data_account);
        let slnd_feed_acc_info = account.into_account_info();
        let aggregator: AggregatorState = switchboard_program::get_aggregator(&slnd_feed_acc_info).unwrap();
        let round_result: RoundResult = switchboard_program::get_aggregator_result(&aggregator).unwrap();
        let price = round_result.result.unwrap_or(0f64);
        return price;
    }

    pub fn get_mnde_price(_rpc_client: &RpcClient) -> f64 {
        info!("Get mnde price");
        let mnde_url = "https://api.coingecko.com/api/v3/simple/price?ids=marinade&vs_currencies=usd";
        let body = reqwest::blocking::get(mnde_url).unwrap().text().unwrap();
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        let price = &v["marinade"]["usd"];
        return price.as_f64().unwrap_or(0f64);
    }
}

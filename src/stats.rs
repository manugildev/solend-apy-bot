use std::{ops::Mul, str::FromStr};
use log::info;
use serde::{Serialize, Deserialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey};
use spl_token_lending::state::Reserve;

use crate::{AssetSymbol, PRODUCTION_CONFIG_JSON, utils::ProgramConfig};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub slnd_price: f64,
    pub total_supplied: f64,
    pub total_borrowed: f64,
}

impl Stats {
    pub fn from_assets(rpc_client: &RpcClient, assets: &Vec<AssetSymbol>) -> Stats {
        let program_config : ProgramConfig = serde_json::from_str(PRODUCTION_CONFIG_JSON).unwrap();
        // Get all production PubKeys
        let mut account_pks = Vec::<Pubkey>::new();
        for &asset_symbol in assets {
            let reserve_json= program_config.markets[0].reserves.iter().find(|e| e.asset == asset_symbol).unwrap();
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
        return Stats {
            slnd_price,
            total_supplied,
            total_borrowed,
        };
    }

    // TODO: Move this somewherelse as soon as IDO is finished and SLND is minted
    const IDO_USDC_ADDRESS: &'static str = "GkV2kxCeAU5qZEPqHbKXdqTByLapzKfTWqQRzCz6S3n1"; 
    pub fn get_slnd_price(rpc_client: &RpcClient) -> f64 {
        info!("Calculate slnd price for ido");
        let ido_pk = Pubkey::from_str(&Self::IDO_USDC_ADDRESS).unwrap();
        let account= rpc_client.get_token_account_balance(&ido_pk).unwrap();
        let total_slnd = 4_000_000 as f64;
        return (account.ui_amount.unwrap() as f64) / total_slnd;
    }
}
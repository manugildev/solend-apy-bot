mod apy;
mod bot;
mod db;
mod utils;

use actix_files::Files;
use actix_web::{
    App,
    dev::Server,
    get,
    HttpResponse,
    HttpServer,
    middleware::Logger,
    Responder,
    rt,
    web,
};
use clap::{load_yaml, App as ClapApp};
use chrono::Duration as chrono_Duration;
use dotenv::dotenv;
use lazy_static::lazy_static;
use log::{info, error};
use solana_client::rpc_client::RpcClient;
use std::{
    path::Path,
    path::PathBuf,
    str::FromStr,
    sync::mpsc,
    thread,
    time::Duration
};

use apy::APY;
use db::DataType;
use db::Database;
use bot::TwitterBot;
use bot::ScreenshotBot;
use utils::AssetSymbol;
use utils::ChartData;
use utils::config;

const RPC_URL: &str = "https://api.mainnet-beta.solana.com";
const PRODUCTION_CONFIG_JSON: &str = include_str!("assets/production.json");
const _DEVNET_CONFIG_JSON: &str = include_str!("assets/devnet.json");

lazy_static! {
#[rustfmt::skip]
static ref PRODUCTION_ASSETS: Vec<AssetSymbol> = [
        (AssetSymbol::SOL),
        (AssetSymbol::USDC),
        (AssetSymbol::USDT),
        (AssetSymbol::ETH),
        (AssetSymbol::BTC),
        (AssetSymbol::SRM),
        (AssetSymbol::FTT),
        (AssetSymbol::RAY),
        (AssetSymbol::SBR),
        (AssetSymbol::MER),
    ].iter().cloned().collect();
}

//=========================================================================================
// API
//=========================================================================================
#[get("/apy")]
async fn apy_route() -> impl Responder {
    let client = RpcClient::new_with_timeout(RPC_URL.to_string(), Duration::from_secs(120));
    let result = APY::from_assets(&client, &PRODUCTION_ASSETS);
    HttpResponse::Ok().json(&result)
}

#[get("/apy/{asset_symbol}")]
async fn apy_asset_route(param: web::Path<String>) -> impl Responder {
    let client = RpcClient::new_with_timeout(RPC_URL.to_string(), Duration::from_secs(120));
    let asset_symbol = AssetSymbol::from_str(&param.to_uppercase()).unwrap();
    let apy = APY::from_asset(&client, asset_symbol);
    HttpResponse::Ok().json(&apy)
}

#[get("/chart_data")]
async fn chart_data() -> impl Responder {
    // TODO: Find a way to reuse the runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut result = Vec::new();
    let async_block  = async { 
        let database = Database::from_config(utils::Config::from_env().unwrap()).await;
        result = database.get_daily_datapoints_as_avg(chrono_Duration::days(7)).await;
    };
    rt.block_on(async_block);

    // Process data for Vue charting
    let mut chart_data_borrow_vec : Vec<ChartData> = Vec::new();
    let mut chart_data_supply_vec : Vec<ChartData> = Vec::new();
    for &asset_symbol in PRODUCTION_ASSETS.iter() {
        let mut data_points_borrow = Vec::new();
        let mut data_points_supply = Vec::new();
        let index : usize = result.iter().position(|e| { e.name == asset_symbol } ).unwrap();
        for s in &result[index].supply {
            let supply_value = f64::trunc(s[1].parse::<f64>().unwrap() * 10000.0) / 100.0;
            data_points_supply.push((s[0].to_string(), supply_value));
        }
        for s in &result[index].borrow {
            let borrow_value = f64::trunc(s[1].parse::<f64>().unwrap() * 10000.0) / 100.0;
            data_points_borrow.push((s[0].to_string(), borrow_value));
        }
        let chart_data_borrow = ChartData { name: asset_symbol, data: data_points_borrow, };
        let chart_data_supply= ChartData { name: asset_symbol, data: data_points_supply, };
        chart_data_supply_vec.push(chart_data_supply);
        chart_data_borrow_vec.push(chart_data_borrow);
    }

    HttpResponse::Ok().json((&chart_data_supply_vec, &chart_data_borrow_vec))
}

//=========================================================================================
// ENTRY POINT
//=========================================================================================
fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "solend_apy_bot=info");
    env_logger::init();
    dotenv().ok();

    let yaml = load_yaml!("assets/cli.yml");
    let matches = ClapApp::from_yaml(yaml).get_matches();
    info!("Arguments parsed");

    let config = utils::Config::from_env().unwrap();
    info!("Configuration imported from .env");

    // Start WebServer
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = server_app(tx);
    });
    let srv = rx.recv().unwrap();

    if matches.is_present("server") {
        let (ctrlc_tx, ctrlc_rx) = mpsc::channel();

        ctrlc::set_handler(move || ctrlc_tx.send(())
            .expect("Could not send signal on channel"))
            .expect("Error setting CTRL-C handler");

        // Keep server alive for debugging purposes
        ctrlc_rx.recv().expect("Could not receive singal from channel.");
        // Close WebServer
        rt::System::new("").block_on(srv.stop(true));
        info!("Server closed");
        return;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    // Save Data in database
    {
        if let Some(data_type) = matches.value_of("data") {
            let async_block = async {
                let database = Database::from_config(config.clone()).await;
                let data_type = DataType::from_str(&data_type).unwrap();
                database.save_apys_in_database(config.clone(), data_type).await;
            };
            rt.block_on(async_block);
        }
    }

    // Take Screenshot
    let mut image_paths = Vec::<PathBuf>::new();
    {
        let screenshot_bot = ScreenshotBot::from_config(config.clone()).unwrap();
        if matches.is_present("screenshot") {
            image_paths.push(screenshot_bot.take_screenshot("/".to_string(), ".b-aspect-content".to_string()).unwrap());
        }
        if matches.is_present("charts") {
            image_paths.push(screenshot_bot.take_screenshot("/charts".to_string(), ".row.supply_chart".to_string()).unwrap());
            image_paths.push(screenshot_bot.take_screenshot("/charts".to_string(), ".row.borrow_chart".to_string()).unwrap());
        }
    }

    // Close WebServer
    rt::System::new("").block_on(srv.stop(true));
    info!("Server closed");

    // Tweet screenshot
    {
        if matches.is_present("twitter") {
            if !matches.is_present("charts") && !matches.is_present("screenshot") {
                error!("--twitter needs to be called with either --charts or --screenshot");
            } else {
                let twitter_bot = TwitterBot::from_config(config.clone());
                let async_block = async {
                    for image_path in image_paths {
                        twitter_bot.tweet(&image_path).await.unwrap();
                    }
                };
                rt.block_on(async_block);
            }
        }
    }

    info!("Closing solend-apy-bot successfully");
}

fn server_app(tx: mpsc::Sender<Server>) -> std::io::Result<()> {
    let mut sys = rt::System::new("server_system");

    let config = config::Config::from_env().unwrap();
    let url = format!("http://{}:{}", config.server.host, config.server.port);

    let folder_name  = Path::new(env!("CARGO_MANIFEST_DIR")).join("web/dist/");
    // srv is server controller type, `dev::Server`
    let srv = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(apy_route)
            .service(apy_asset_route)
            .service(chart_data)
            .service(Files::new("/", folder_name.clone()).index_file("index.html"))
            //.service(Files::new("/", ).index_file("index.html"))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run();

    info!("Starting server at {}", url);
    tx.send(srv.clone()).unwrap();
    sys.block_on(srv)
}
use chrono::offset::Utc;
use chrono::DateTime;
use mongodb::{
    bson::{to_document, doc},
    Client,
    Collection,
    options::{ClientOptions, FindOptions},
};
use log::info;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use std::fmt;

use crate::config::Config;
use crate::apy::APY;

pub struct Database {
    client: Client,
    database_name: &'static str,
}

impl Database {
    pub async fn from_config(config: Config) -> Self {
        let dbconfig = config.mongodb;

        // Parse a connection string into an options struct.
        let url = format!(
            "mongodb+srv://{}:{}@{}/{}?retryWrites=true&w=majority",
            dbconfig.user, dbconfig.pass, dbconfig.server, dbconfig.dbname
        );
        let mut client_options = ClientOptions::parse(url).await.unwrap();
        client_options.app_name = Some("solend-apy-bot".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options).unwrap();

        let database_name = "solend-database";

        return Self {
            client: client,
            database_name: database_name,
        }
    }

    async fn insert_apys(&self, apys: &Vec<APY>, data_type: DataType) {
        let new_doc = APYDataPoint {
            date: Utc::now(),
            data_type: data_type,
            apys: apys.clone(),
        };
        let collection_name = format!("data_{}", data_type.to_string().to_lowercase());
        let apys = self.client.database(self.database_name).collection(collection_name.as_str());

        let insert_data = apys.insert_one(to_document(&new_doc).unwrap(), None).await.unwrap();
        info!("Data inserted in {}/{}: {}", self.database_name, collection_name, insert_data.inserted_id);
    }

    pub async fn get_datapoints(&self, limit: i64, data_type: DataType) -> Vec<APYDataPoint> {
        use futures::stream::TryStreamExt;

        let collection_name = format!("data_{}", data_type.to_string().to_lowercase());
        let collection : Collection<APYDataPoint> = self.client.database(self.database_name).collection(collection_name.as_str());
        // Query the books in the collection with a filter and an option.
        let filter = doc! { };
        let find_options = FindOptions::builder().sort(doc! { "date": -1 }).limit(limit).build();
        let mut cursor = collection.find(filter, find_options).await.unwrap();

        let mut result = Vec::<APYDataPoint>::new();
        while let Some(data_point) = cursor.try_next().await.unwrap() {
            result.push(data_point);
        }
        return result;
    }

    pub async fn save_apys_in_database(&self, config: Config, data_type: DataType) {
        let request_url = format!("http://{}:{}/apy", config.server.host, config.server.port);
        let res = reqwest::get(request_url).await.unwrap();
        let body = res.text().await.unwrap();
        let result: Vec<APY> = serde_json::from_str(&body.as_str()).unwrap();
        self.insert_apys(&result, data_type).await;
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

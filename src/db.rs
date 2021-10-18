use chrono::Datelike;
use chrono::offset::Utc;
use chrono::DateTime;
use chrono::Duration;
use chrono::TimeZone;
use mongodb::{
    Client,
    Collection,
    bson::{self, doc, from_document, to_document},
    options::{ClientOptions, FindOptions}
};
use log::info;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use std::fmt;

use crate::config::Config;
use crate::apy::APY;
use crate::AssetSymbol;

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

    #[allow(unused)]
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

    pub async fn get_daily_datapoints_as_avg(&self, days_back: Duration) -> Vec<APYDataPointAggregatedString> {
        use futures::stream::TryStreamExt;
        let now = Utc::now();
        let today= Utc.ymd(now.year(), now.month(), now.day()).and_hms(0, 00, 00);
        let from_date = today.checked_sub_signed(days_back).unwrap().checked_add_signed(Duration::seconds(1)).unwrap();
        let pipeline = vec![
            doc! {
                "$match": { "date": { "$gte": from_date.to_string(), "$lte": today.to_string() } }
            },
            doc! {
                "$unwind": { "path": "$apys" }
            },
            doc! {
                "$addFields": {
                    "date": {
                        "$dateFromString": { "dateString": "$date" }
                    }
                }
            },
            doc! {
                "$group": {
                    "_id": {
                        "asset": "$apys.asset",
                        "date": {
                            "$dateFromParts": {
                                "year": { "$year": "$date" },
                                "month": { "$month": "$date" },
                                "day": { "$dayOfMonth": "$date" }
                            }
                        }
                    },
                    "date": {
                        "$first": {
                            "$dateFromParts": {
                                "year": { "$year": "$date" },
                                "month": { "$month": "$date" },
                                "day": { "$dayOfMonth": "$date" }
                            }
                        }
                    },
                    "avg_borrow": { "$avg": "$apys.borrow" },
                    "avg_supply": { "$avg": "$apys.supply" },
                    "avg_price": { "$avg": "$apys.price" }
                }
            },
            doc! {
                "$sort": { "date": 1 }
            },
            doc! {
                "$group": {
                    "_id": "$_id.asset",
                    "name": { "$first": "$_id.asset" },
                    "supply": {
                        "$push": { "data": [ "$_id.date", "$avg_supply" ] }
                    },
                    "borrow": {
                        "$push": { "data": [ "$_id.date", "$avg_borrow" ] }
                    },
                }
            }
        ];

        let collection_name = format!("data_{}", DataType::HOUR.to_string().to_lowercase());
        let data_points: Collection<APYDataPoint> = self.client.database(self.database_name).collection(collection_name.as_str());
        let mut results = data_points.aggregate(pipeline, None).await.unwrap();
        let mut docs = Vec::new();
        while let Some(result) = results.try_next().await.unwrap() {
            let doc: APYDataPointAggregated = from_document(result).unwrap();
            docs.push(APYDataPointAggregatedString::from(doc));
        }
        return docs;
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APYDataPointAggregatedData {
    pub data: (bson::DateTime, f64),
}

impl From<APYDataPointAggregatedData> for Vec<String> {
    fn from(item: APYDataPointAggregatedData) -> Self {
        return vec![item.data.0.to_string(), item.data.1.to_string()];
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APYDataPointAggregated {
    pub name: AssetSymbol,
    pub supply: Vec<APYDataPointAggregatedData>,
    pub borrow: Vec<APYDataPointAggregatedData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APYDataPointAggregatedString {
    pub name: AssetSymbol,
    pub supply: Vec<Vec<String>>,
    pub borrow: Vec<Vec<String>>,
}

impl From<APYDataPointAggregated> for APYDataPointAggregatedString {
    fn from(item: APYDataPointAggregated) -> Self {
        return APYDataPointAggregatedString {
            name: item.name,
            supply: item.supply.iter().map(|e| Vec::<String>::from(e.clone())).collect(),
            borrow: item.borrow.iter().map(|e| Vec::<String>::from(e.clone())).collect(),
        }
    }
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

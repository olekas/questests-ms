use mongodb::{
    bson::oid::ObjectId,
    options::ClientOptions
};

use serde::{Deserialize, Serialize};
use std::env;
use crate::db;

use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokenInfo {
    pub expiration: DateTime<Utc>;
    pub client_info: String;
    pub token: String;
    pub replaced_token: Option<String>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,    
    pub first_name: String,
    pub last_name: String,
    pub summary: String,
    pub login: String,
    pub password: String,
    pub tokens: Option<Vec<TokenInfo>>
}

impl db::connect::MongodbOptionsProvider for User {
    fn id(&self) -> Option<ObjectId> { self.id.clone() }
    fn connection_string() -> String { db::connect::default_connection_string() }
    fn client_options(options: &ClientOptions) -> ClientOptions { db::connect::default_client_options(options) }
    fn database_name() -> String { env::var("MONGODB_DATABASE_NAME").unwrap() }
    fn collection_name() -> String { String::from("users") } 
}
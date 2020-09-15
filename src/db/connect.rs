use mongodb::{
    bson::oid::ObjectId, 
    error::Result,
    options::{ ClientOptions, Credential }
};

use std::env;

pub trait MongodbOptionsProvider {
    fn id(&self) -> Option<ObjectId>;
    fn connection_string() -> String;
    fn client_options(options: &ClientOptions) -> ClientOptions;
    fn database_name() -> String;
    fn collection_name() -> String;
}

const DB_APPLICATION_NAME:&'static str = "MongoDB Test App";

pub fn default_connection_string() -> String {
    format!("mongodb://{}:{}@{}", 
        env::var("MONGODB_USER").unwrap(),
        env::var("MONGODB_PASSWORD").unwrap(),
        env::var("MONGODB_URL").unwrap()
    )
}

pub fn default_client_options(options: &ClientOptions) -> ClientOptions {
    let mut options_clone = options.clone();
    options_clone.app_name = Some(String::from(DB_APPLICATION_NAME));
    options_clone
}


use mongodb::{
    bson::oid::ObjectId,
    options::ClientOptions
};

use serde::{Deserialize, Serialize};
use std::env;
use crate::db;

use chrono::{DateTime, Utc};
use std::time::Duration;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QuestionaryCast {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,    
    pub pattern_refid: Option<bson::oid::ObjectId>,    
    pub user_refid: Option<bson::oid::ObjectId>,    
    pub executed: DateTime<Utc>,
    pub questions: Vec<QuestionaryCast>,
    pub time_taken: Duration,
    pub score: f32
}

impl db::connect::MongodbOptionsProvider for QuestionaryCast {
    fn id(&self) -> Option<ObjectId> { self.id.clone() }
    fn connection_string() -> String { db::connect::default_connection_string() }
    fn client_options(options: &ClientOptions) -> ClientOptions { db::connect::default_client_options(options) }
    fn database_name() -> String { env::var("MONGODB_DATABASE_NAME").unwrap() }
    fn collection_name() -> String { String::from("questionary_cast") } 
}
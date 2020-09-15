use mongodb::{
    bson::oid::ObjectId,
    options::ClientOptions
};

use serde::{Deserialize, Serialize};
use std::env;
use crate::db;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QuestionCast {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,    
    pub source: String,
    pub subject: String,
    pub topics: Vec<String>,
    pub tags: Vec<String>,
    pub difficulty: f32,
    pub caption: String,
    pub description: String,
    pub selected_answers: Vec<u32>,
    pub presented_answers: Vec<QuestionAnswer>,
    pub score: f32
}


impl db::connect::MongodbOptionsProvider for QuestionCast {
    fn id(&self) -> Option<ObjectId> { self.id.clone() }
    fn connection_string() -> String { db::connect::default_connection_string() }
    fn client_options(options: &ClientOptions) -> ClientOptions { db::connect::default_client_options(options) }
    fn database_name() -> String { env::var("MONGODB_DATABASE_NAME").unwrap() }
    fn collection_name() -> String { String::from("question_cast") } 
}
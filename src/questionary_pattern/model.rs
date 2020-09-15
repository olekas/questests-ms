use mongodb::{
    bson::oid::ObjectId,
    options::ClientOptions
};

use serde::{Deserialize, Serialize};
use std::env;
use crate::db;

use std::time::Duration;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QuestionQuery {
    pub subject_qry: String,
    pub topic_qry: String,
    pub tag_qry: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QuestionaryPattern {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,    
    pub description: String,
    pub created: DateTime<Utc>,
    pub created_by: Option<bson::oid::ObjectId>,    
    pub question_query: Option<QuestionQuery>,
    pub question_selection: Option<Vec<bson::oid::ObjectId>>,
    pub number_of_questions: u32,
    pub min_number_of_questions: u32,
    pub time_limit: Duration
}



impl db::connect::MongodbOptionsProvider for QuestionaryPattern {
    fn id(&self) -> Option<ObjectId> { self.id.clone() }
    fn connection_string() -> String { db::connect::default_connection_string() }
    fn client_options(options: &ClientOptions) -> ClientOptions { db::connect::default_client_options(options) }
    fn database_name() -> String { env::var("MONGODB_DATABASE_NAME").unwrap() }
    fn collection_name() -> String { String::from("questionary_pattern") } 
}
use mongodb::{
    bson::oid::ObjectId,
    options::ClientOptions
};

use serde::{Deserialize, Serialize};
use std::env;

use crate::db;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QuestionAnswer {
    pub answer_text: String,
    pub rate: f32
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Question {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,    
    pub source: Option<String>,
    pub subject: String,
    pub topics: Vec<String>,
    pub tags: Vec<String>,
    pub difficulty: f32,
    pub description: String,
    pub to_choose: i32,
    pub to_show: Option<u32>,
    pub answers: Vec<QuestionAnswer>
}

impl db::connect::MongodbOptionsProvider for Question {
    fn id(&self) -> Option<ObjectId> { self.id.clone() }
    fn connection_string() -> String { db::connect::default_connection_string() }
    fn client_options(options: &ClientOptions) -> ClientOptions { db::connect::default_client_options(options) }
    fn database_name() -> String { env::var("MONGODB_DATABASE_NAME").unwrap() }
    fn collection_name() -> String { String::from("questions") } 
}

impl Question {
    pub fn new(subject: String, topics: &Vec<String>, difficulty: f32, description: String, to_choose: i32) -> Question {
        Question {
            id: None,
            source: None,
            subject: subject,
            topics: topics.clone(),
            tags: vec![],
            difficulty: difficulty,
            description: description,
            to_choose: to_choose,
            to_show: None,
            answers: vec![]
        }
    }

    pub fn append_answer(&mut self, answer: &QuestionAnswer) {
        self.answers.push(answer.clone())
    }
}
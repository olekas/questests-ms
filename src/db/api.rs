use mongodb::{
    bson::{ self, Bson, Document, doc, oid::ObjectId }, 
    options::{ ClientOptions, FindOptions }, 
    Client, 
    Collection
};
use serde::{ Deserialize, Serialize };
use futures::stream::StreamExt;
use super::connect::MongodbOptionsProvider;

use crate::errors::Error;

type Result<T> = std::result::Result<T, Error>;

async fn get_collection<T>() -> Result<Collection>
    where T: MongodbOptionsProvider {
    let client_options = ClientOptions::parse(&T::connection_string().to_string()).await?;
    let client = Client::with_options(T::client_options(&client_options))?;
    Ok(client
        .database(&T::database_name().to_string())
        .collection(&T::collection_name().to_string()))
}

pub async fn find<T>(filter: Option<Document>, options: Option<Document>) -> Result<Vec<T>> 
    where for<'de> T: Deserialize<'de> + MongodbOptionsProvider {
        
    let mut found = Vec::new();
    match get_collection::<T>().await {
        Ok(collection) => {
            let find_options = FindOptions::builder().sort(options).build();
            let mut cursor = collection.find(filter, find_options).await?;
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(record_doc) => {
                        found.push(match bson::from_bson(Bson::Document(record_doc)) {
                            Ok(res) => res,
                            Err(e) => return Err(mongodb::error::Error::from(e).into()),                
                        });        
                    }
                    Err(e) => return Err(e.into())
                }
            }            
        },
        Err(e) => return Err(e.into())
    };

    Ok(found)
}

pub async fn get<T>(id: String) -> Result<Option<T>> 
    where for<'de> T: Deserialize<'de> + MongodbOptionsProvider {
        let object_id = match ObjectId::with_string(&id.to_string()){
            Ok(as_object_id) => as_object_id,
            Err(_) => return Err(Error::new(&*format!("Identifer '{}' is not a valid ObjectId", id)))
        };

        match get_collection::<T>().await {
        Ok(collection) => {
            match collection.find_one(doc! {"_id": object_id}, None).await {
                Ok(result) => {
                    return match result {
                        Some(result_doc) => Ok(match bson::from_bson(Bson::Document(result_doc)) {
                            Ok(res) => Some(res),
                            Err(e) => return Err(mongodb::error::Error::from(e).into()),
                        }),
                        None => Ok(None),
                    };                    
                },
                Err(e) => return Err(e.into()),
            }                
        },
        Err(e) => return Err(e.into())
    }
}

pub async fn insert<T>(data: &T) -> Result<ObjectId> 
    where T: Serialize + MongodbOptionsProvider {

    let inserted = match get_collection::<T>().await {
        Ok(collection) => match bson::to_document(data) {
            Ok(model_doc) => collection.insert_one(model_doc, None).await?,
            Err(e) => return Err(mongodb::error::Error::from(e).into()),
        },
        Err(e) => return Err(e.into())
    };
    
    Ok(bson::from_bson(inserted.inserted_id).unwrap())
}

pub async fn update<T>(data: &T) -> Result<()> 
    where T: Serialize + MongodbOptionsProvider {

    let inserted = match get_collection::<T>().await {
        Ok(collection) => match bson::to_document(data) {
            Ok(model_doc) => collection.replace_one(doc! {"_id": data.id().unwrap()}, model_doc, None).await?,
            Err(e) => return Err(mongodb::error::Error::from(e).into()),
        },
        Err(e) => return Err(e.into())
    };
    
    Ok(())
}

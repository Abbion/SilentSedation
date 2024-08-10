use core::fmt;
use std::str::FromStr;

use mongodb::{ options::ClientOptions, Client, Database };
use user_data::UserDataCollection;

mod user_data;

use serde::{Serialize, Deserialize};
use bson::{ Document, oid::ObjectId };

fn to_document<T>(data: &T) -> Option<Document>
where
    T: Serialize + Deserialize<'static>,
{
    let json_string = match serde_json::to_string(data) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Struct to json string failed: {}", err);
            return  None;
        }
    };

    let json_value = match serde_json::from_str::<serde_json::Value>(&json_string) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Json to value failed: {}", err);
            return None;
        }
    };

    let bson_document = match bson::to_document(&json_value) {
        Ok(document) => document,
        Err(err) => {
            eprintln!("Value to document failed: {}", err);
            return None;
        }
    };

    Some(bson_document)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UserId {
    _id: ObjectId,
}

impl fmt::Display for UserId {
       fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self._id)
    } 
}

impl UserId {
    pub fn from_str(id_str : &String) -> Option<UserId> {
        let mongo_id = match ObjectId::from_str(&id_str) {
            Ok(id) => id,
            Err(e) => return None
        };

        Some(UserId{_id: mongo_id})
    }
}

pub async fn connect_to_database() -> Database {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await;

    let mut client_options = match client_options {
        Ok(co) => co,
        Err(err) => {
            panic!("client options parse failed: {}", err);
        }
    };

    client_options.app_name = Some("silent_server".to_string());

    let client = Client::with_options(client_options);

    let client = match client {
        Ok(c) => c,
        Err(err) => {
            panic!("client creation faild: {}", err);
        }
    };

    client.database("silent_sedation_db") 
}

async fn get_collection_names(db: &Database) -> Vec<String> {
    match db.list_collection_names(None).await {
        Ok(cn) => cn,
        Err(e) => {
            eprintln!("Collection names request failed: {}", e);
            Vec::new()
        }
    }
}

pub async fn get_collections(db: &Database) -> UserDataCollection {
    let collection_names = get_collection_names(db).await;

    if collection_names.is_empty() {
        panic!("collection names returned empty!");
    }

    for collection_name in collection_names {
        if collection_name.eq("user_data") {
            return UserDataCollection::new(db);
        }
    }
    
    //TODO: return option
    UserDataCollection::new(db)
}
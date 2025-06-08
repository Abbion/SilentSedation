// Refactor 4.0
use core::fmt;
use std::str::FromStr;
use mongodb::{ options::ClientOptions, Client, Database };
use user_data::UserDataCollection;
use device_data::DeviceDataCollection;
use device_code_data::DeviceCodeDataCollection;
use serde::{Serialize, Deserialize};
use bson::{ Document, oid::ObjectId };

pub mod error_types;
mod user_data;
mod device_data;
mod device_code_data;

const MONGO_DB_ADDRES : &str = "localhost:27017";
const MONGO_DB_APP_NAME : &str = "user_server";
const APP_DB_NAME : &str = "silent_sedation_db";
pub const USER_COLLECTION_NAME : &str = "user_data";
pub const DEVICE_COLLECTION_NAME : &str = "device_data";
pub const DEVICE_CODE_COLLECTION_NAME : &str = "device_code_data";

pub enum CollectionType {
    UserCollection,
    DeviceCollection,
    DeviceCodeCollection,
}

pub enum Collection {
    User(UserDataCollection),
    Device(DeviceDataCollection),
    DeviceCode(DeviceCodeDataCollection)
}

pub type UserId = DatabaseObjectId;
pub type CardId = i64;
pub type DeviceId = DatabaseObjectId;

pub struct DatabaseReadError(&'static str);

impl CollectionType {
    fn as_str(&self) -> &str {
        match self {
            CollectionType::UserCollection => USER_COLLECTION_NAME,
            CollectionType::DeviceCollection => DEVICE_COLLECTION_NAME,
            CollectionType::DeviceCodeCollection => DEVICE_CODE_COLLECTION_NAME
        }
    }
}

impl fmt::Display for DatabaseReadError {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Database error: {}", self.0)
    }
}

fn to_document<T>(data : &T) -> Option<Document>
where
    T: Serialize + Deserialize<'static>,
{
    let json_string = match serde_json::to_string(data) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("Struct to json string failed: {}", error);
            return  None;
        }
    };

    let json_value = match serde_json::from_str::<serde_json::Value>(&json_string) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Json to value failed: {}", error);
            return None;
        }
    };

    let bson_document = match bson::to_document(&json_value) {
        Ok(document) => document,
        Err(error) => {
            eprintln!("Value to document failed: {}", error);
            return None;
        }
    };

    Some(bson_document)
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DatabaseObjectId {
    _id : ObjectId,
}

impl fmt::Display for DatabaseObjectId {
       fn fmt(&self, formater : &mut fmt::Formatter) -> fmt::Result {
        write!(formater, "{}", self._id)
    } 
}

impl DatabaseObjectId {
    pub fn new(object_id : ObjectId) -> DatabaseObjectId {
        DatabaseObjectId { _id: object_id }
    }

    pub fn from_str(id_str : &String) -> Option<DatabaseObjectId> {
        let mongo_id = match ObjectId::from_str(&id_str) {
            Ok(id) => id,
            Err(_) => return None
        };

        Some(DatabaseObjectId{_id: mongo_id})
    }

    pub fn compare(&self, cmp : &DatabaseObjectId) -> bool {
        return self._id == cmp._id;
    }
}

pub async fn connect_to_database() -> Database {
    let parse_result = ClientOptions::parse(format!("mongodb://{}", MONGO_DB_ADDRES)).await;

    let mut client_options = match parse_result {
        Ok(options) => options,
        Err(error) => {
            panic!("Client options parse failed: {}", error);
        }
    };

    client_options.app_name = Some(String::from(MONGO_DB_APP_NAME));

    let result = Client::with_options(client_options);

    let client = match result {
        Ok(client) => client,
        Err(error) => {
            panic!("client creation faild: {}", error);
        }
    };

    client.database(APP_DB_NAME) 
}

async fn get_collection_names(db : &Database) -> Vec<String> {
    match db.list_collection_names(None).await {
        Ok(collection_name) => collection_name,
        Err(error) => {
            eprintln!("Collection names request failed: {}", error);
            Vec::new()
        }
    }
}

pub async fn get_collection(db : &Database, collection_type : CollectionType) -> Result<Collection, DatabaseReadError> {
    let collection_names = get_collection_names(db).await;

    if collection_names.is_empty() {
        panic!("collection names returned empty!");
    }

    let selected_collection_name = collection_type.as_str();

    if collection_names.contains(&selected_collection_name.to_string())
    {
        match collection_type {
            CollectionType::UserCollection => return Ok(Collection::User(UserDataCollection::new(db))),
            CollectionType::DeviceCollection => return Ok(Collection::Device(DeviceDataCollection::new(db))),
            CollectionType::DeviceCodeCollection => return Ok(Collection::DeviceCode(DeviceCodeDataCollection::new(db)))
        }
    }

    Err(DatabaseReadError("Cound not find collection!"))
}
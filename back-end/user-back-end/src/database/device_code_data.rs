use chrono::Duration;
use mongodb::{ Collection, Database };
use bson::{ doc, oid::ObjectId, DateTime, Document };
use serde::{Deserialize, Serialize};
use crate::{code_generator::Code, constants::CODE_EXPIRATION_TIME_IN_MIN};
use futures::stream::StreamExt;

use super::{to_document, DeviceId, DEVICE_CODE_COLLECTION_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCodeEntry {
    device_id : ObjectId,
    code : String,
    time_stamp : DateTime,
}

pub struct DeviceCodeDataCollection {
    collection : Collection<Document>
}

impl DeviceCodeDataCollection {
    pub fn new(db: &Database) -> DeviceCodeDataCollection {
        DeviceCodeDataCollection{ collection : db.collection::<Document>(DEVICE_CODE_COLLECTION_NAME) }
    }

    pub async fn assign_code_to_device(&self, code : Code, device_id : DeviceId) -> Option<String> {
        let device_code_entry = DeviceCodeEntry {
            device_id: device_id._id,
            code : code.as_string(),
            time_stamp : DateTime::from_chrono(chrono::Utc::now())
        };

        let device_code_entry_document = match to_document(&device_code_entry) {
            Some(document) => document,
            None => {
                eprintln!("Assign code to device failed: device code entry can't be converted into a document");
                return None;
            }
        };

        let insertion_result = self.collection.insert_one(device_code_entry_document, None).await;

        match insertion_result {
            Ok(_) => {
                return Some(code.as_string());
            }
            Err(error) => {
                eprintln!("Assign code to device failed db insertion fail, error: {}", error);
                return None;
            }
        }
    }

    pub async fn remove_device_expired_codes(&self) -> Option<Vec<Code>> {
        let time_stamp = chrono::Utc::now();
        let threshold_time_stamp = time_stamp - Duration::minutes(CODE_EXPIRATION_TIME_IN_MIN);
        let bison_time = DateTime::from_chrono(threshold_time_stamp);

        let query = doc! { "time_stamp": { "$lt" : bison_time } };

        // Configure the batch size if this takes to long
        let mut cursor = match self.collection.find(query.clone(), None).await {
            Ok(db_cursor) => db_cursor,
            Err(error) => {
                eprintln!("Getting expired timestamps failed: {}", error);
                return None;
            }
        };

        let mut codes : Vec<Code> = Vec::new();
        while let Some(document) = cursor.next().await {
            let code = match document {
                Ok(doc) => {
                    let code_str = doc.get_str("code");
                    if code_str.is_ok() {
                        Code::from_string(code_str.unwrap().to_string())
                    } 
                    else {
                        None
                    }
                }
                Err(error) =>{
                    eprintln!("Cursor failed while geting codes: {}", error);
                    None
                }
            };

            if let Some(code) = code {
                codes.push(code);
            }
        }

        if codes.len() > 0 {
            let result = self.collection.delete_many(query, None).await;
            if let Err(error) = result {
                eprintln!("Removing expired codes failed: {}", error);
            }
        }

        Some(codes)
    }

    pub fn get_device_code(&self, device : DeviceId) -> Code {
        

        Code::new()
    }
/*
    pub fn get_device_id_by_code(code : Code) {

    }

    pub fn release_device(device : DeviceId) {

    }
*/
}
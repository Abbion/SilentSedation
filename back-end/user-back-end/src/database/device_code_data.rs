use mongodb::{ Collection, Database };
use bson::{ oid::ObjectId, DateTime, Document };
use serde::{Deserialize, Serialize};
use crate::code_generator::Code;

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
            time_stamp : DateTime::now()
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
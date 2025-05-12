use mongodb::{ options::FindOneOptions, Collection, Database };
use bson::{ doc, oid::ObjectId, Bson, Document };
use serde::{Deserialize, Serialize};
use crate::{communication::requests::RegisterDeviceRequest, utils::{device_states::{ DeviceStateValue, DeviceState }, device_types::DeviceTypeValue}};

use super::{to_document, DeviceId, DEVICE_COLLECTION_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceEntry {
    _id : ObjectId,
    device_type : DeviceTypeValue,
    device_master : Option<ObjectId>,
    device_state : DeviceStateValue
}

pub struct DeviceDataCollection {
    collection : Collection<Document>
}

impl DeviceDataCollection {
    pub fn new(db: &Database) -> DeviceDataCollection {
        DeviceDataCollection{ collection : db.collection::<Document>(DEVICE_COLLECTION_NAME) }
    }

    pub async fn is_device_pressent(&self, device_id : DeviceId) -> Option<bool> {
        let filter = match to_document(&device_id) {
            Some(document) => document,
            None => { return None }
        };

        let find_options = FindOneOptions::builder().projection(doc! {"_id": 1}).build();
        let find_result = self.collection.find_one(filter, find_options).await;

        let find_document = match find_result {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Getting device id failed: {}", error);
                return None;
            }
        };

        let device_id_result = match find_document {
            Some(document) => bson::from_bson::<DeviceId>(Bson::Document(document)),
            None => { return Some(false); }
        };

        let db_device_id = match device_id_result {
            Ok(id) => id,
            Err(error) => {
                eprintln!("Parsing device id failed: {}", error);
                return None;
            }
        };

        Some(db_device_id.compare(&device_id))
    }

    pub async fn register_device(&self, register_device_data: RegisterDeviceRequest) -> bool {
        let device_id = DeviceId::from_str(&register_device_data.device_id);

        let device_id = match device_id {
            Some(id) => id,
            None => {
                eprintln!("Registering device failed: Device id ({}) can't be converted!", register_device_data.device_id);
                return false;
            }
        };

        let device_entry = DeviceEntry{ _id : device_id._id,
                                                     device_type : register_device_data.device_type,
                                                     device_master : None,
                                                     device_state : DeviceState::Offline.as_native_value() };

        let device_entry_document = match to_document(&device_entry) {
            Some(document) => document,
            None => {
                eprintln!("Registering device failed: device entry struct can't be converted into a document");
                return false;
            }
        };

        let insertion_result = self.collection.insert_one(device_entry_document, None).await;

        match insertion_result {
            Ok(_) => {
                return true;
            },
            Err(error) => {
                eprintln!("Registering device failed: db insertion fail, error: {}", error);
                return false;
            }
        }
    }

    // fn get_owner_devices()
    // fn get_device_status()
    // fn assing_user_to_device()
    // fn update_device_state()

    //http POST 127.0.0.1:9000/register_device device_id=680908739585f2452bf4cbbe device_type:=3
}


//-----------------------------------------------------------------------
// _id                       | devie_type | device_master | device_state
// 680908739585f2452bf4cbbe
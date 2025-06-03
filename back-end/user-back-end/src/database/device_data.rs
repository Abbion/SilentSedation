use futures::StreamExt;
use mongodb::{ options::{FindOneOptions, FindOptions}, Collection, Database };
use bson::{ doc, oid::ObjectId, Bson, Document };
use serde::{Deserialize, Serialize};
use crate::{communication::requests::RegisterDeviceRequest, enums::web_status::WebStatus, utils::{device_states::{ DeviceState, DeviceStateValue }, device_types::DeviceTypeValue}};

use super::{to_document, CardId, DeviceId, UserId, DEVICE_COLLECTION_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceEntry {
    _id : ObjectId,
    device_type : DeviceTypeValue,
    device_master : Option<ObjectId>,
    card_id :       Option<CardId>,
    device_state : DeviceStateValue
}

#[derive(Serialize)]
pub struct DeviceStatusForCard {
    card_id : CardId,
    status : DeviceStateValue
}

pub struct DeviceDataCollection {
    collection : Collection<Document>
}

impl DeviceDataCollection {
    pub fn new(db: &Database) -> DeviceDataCollection {
        DeviceDataCollection{ collection : db.collection::<Document>(DEVICE_COLLECTION_NAME) }
    }

    pub async fn get_user_device_id_at_card_id(&self, user_id : &UserId, card_id : CardId) -> Option<DeviceId> {
        let filter = doc! {
            "device_master": user_id._id,
            "card_id" : card_id
        };

        let find_options = FindOneOptions::builder().projection(doc! {"_id": 1}).build();
        let find_result = self.collection.find_one(filter, find_options).await;

        let document = match find_result {
            Ok(document) => document,
            Err(error) => {
                eprintln!("Getting device id by user id and card id failed: {}", error);
                return None;
            }
        };

        if let Some(document) = document {
            let device_id = match document.get_object_id("_id") {
                Ok(id) => id,
                Err(error) => {
                    eprintln!("Getting device id by user id and card id failed: {}", error);
                    return None;
                }
            };

            return Some(DeviceId::new(device_id));
        }

        None
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
                                                     card_id : None,
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

    pub async fn assign_master_to_device(self, device_id : &DeviceId, user_id : &UserId, card_id : CardId) -> bool {
        let filter = match to_document(device_id) {
            Some(document) => document,
            None => { return false; }
        };

        let update = doc! { "$set" : { "device_master" : user_id._id, "card_id" : card_id } };
        let update_result = self.collection.update_one(filter, update, None).await;

        match update_result {
            Ok(_) => {
                return true;
            },
            Err(error) => {
                eprintln!("Updateing the master for device failed: {}", error);
                return false;
            }
        }
    }

    pub async fn update_device_status(self, device_id : &DeviceId, status : WebStatus) -> bool {
        let filter = doc! { "_id" : device_id._id };
        let update = doc! { "$set" : { "device_state" : status.as_native_value() } };

        let update_status = self.collection.update_one(filter, update, None).await;

        match update_status {
            Ok(_) => {
                return true;
            },
            Err(error) => {
                eprintln!("Updateing the device status fialed: {}", error);
                return false;
            }
        }
    }

    pub async fn get_devices_status_for_user(self, user_id : &UserId) -> Vec<DeviceStatusForCard> {
        let mut devices_status = Vec::<DeviceStatusForCard>::new();

        let filter = match to_document(user_id) {
            Some(filter) => filter,
            None => { 
                eprintln!("Get devices status: Filter cannot be constructed!");
                return devices_status;
            }
        };

        let find_options = FindOptions::builder().projection(doc! {"card_id": 1, "device_state" : 1}).build();
        let find_result = self.collection.find(filter, find_options).await;

        // Configure the batch size if this takes to long
        let mut cursor = match find_result {
            Ok(cursor) => cursor,
            Err(error) => {
                eprintln!("Error: Get device status cursor error: {}", error);
                return devices_status;
            }
        };

        while let Some(document) = cursor.next().await {
            let device_status = match document {
                Ok(result) => {
                    let card_id = result.get_i64("card_id");
                    let status = result.get_i32("device_state");

                    if card_id.is_ok() && status.is_ok() {
                        Some(DeviceStatusForCard { card_id : card_id.unwrap(), status : status.unwrap() })
                    } 
                    else {
                        None
                    }
                }
                Err(_) => {
                    None
                }
            };

            if device_status.is_some() {
                devices_status.push(device_status.unwrap());
            }
        }

        devices_status
    }

    pub async fn put_all_devices_offline(self) -> bool {
        let online_status = WebStatus::as_native_value(&WebStatus::ONLINE);
        let offline_status = WebStatus::as_native_value(&WebStatus::OFFLINE);

        let filter = doc! { "device_state" : online_status };
        let update = doc! { "$set" : { "device_state" : offline_status } };

        let update_status = self.collection.update_many(filter, update, None).await;

        match update_status {
            Ok(_) => {
                return  true;
            },
            Err(error) => {
                eprintln!("Putting devices to offline failed: {}", error);
                return false;
            }
        }
    }
}
// Rework 3.0

use argon2::{ password_hash::{PasswordHash, PasswordVerifier}, Argon2};
use mongodb::{ options::FindOneOptions, Collection, Database };
use bson::{ doc, Bson, Document };
use std::cmp::{ max, min };
use crate::communication::requests::CardData;
use crate::communication::responses::CreateCardResponse;
use crate::communication::{ requests, responses };
use crate::database::{ to_document, UserId };
use crate::utils::device_types::{ DeviceType, ShockCallerData };
use crate::constants;
use super::error_types::DatabaseError;
use super::{CardId, USER_COLLECTION_NAME};

pub struct UserDataCollection {
    collection : Collection<Document>
}

impl UserDataCollection {
    pub fn new(db : &Database) -> UserDataCollection {
        UserDataCollection{ collection : db.collection::<Document>(USER_COLLECTION_NAME) }
    }

    pub async fn get_user_id(&self, login_data : requests::LoginUserRequest) -> Option<UserId> {
        let filter = doc! { "username" : login_data.username };
        let find_options = FindOneOptions::builder().projection(doc! {"_id": 1, "password" : 1}).build();
        let find_result = self.collection.find_one(filter, find_options).await;
    
         let find_document = match find_result {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Getting user id failed: {}", error);
                return None;
            }
        };

        let (user_id, password) = match find_document {
            Some(document) => {
                let id = match document.get_object_id("_id") {
                    Ok(id) => id,
                    Err(error) => {
                        eprintln!("Getting user id from result failed: {}", error);
                        return None;
                    }
                };

                let password = match document.get_str("password") {
                    Ok(password) => password.to_string(),
                    Err(error) => {
                        eprintln!("Getting user password from result failed: {}", error);
                        return None;
                    }
                };

                (id, password)
            },
            None => { return None; }
        };

        let parsed_hash = match PasswordHash::new(&password) {
            Ok(password) => password,
            Err(error) => {
                eprintln!("Getting user password hashing failed: {}",error);
                return None;
            }
        };

        if Argon2::default().verify_password(login_data.password.as_bytes(), &parsed_hash).is_ok() {
            return Some(UserId::new(user_id));
        }

        None
    }

    pub async fn get_user_page_info(&self, user_id : UserId) -> Option<responses::GetUserPageInfoResponse> {
        let filter = match to_document(&user_id) {
            Some(document) => document,
            None => { return None; }
        };

        let find_options = FindOneOptions::builder().projection(doc! {"username": 1, "cards.id": 1}).build();
        let find_result = self.collection.find_one(filter, find_options).await;

        let find_document = match find_result {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Getting user basic info failed: {}", error);
                return None;
            }
        };

        match find_document {
            Some(data) => {
                let username = match data.get_str("username") {
                    Ok(result) => result.to_string(),
                    Err(_) => { return None; }
                };

                let cards = match data.get_array("cards") {
                    Ok(result) => result,
                    Err(_) => { return None; }
                };

                let mut ids_vec = Vec::new();

                for card in cards {
                    match card.as_document() {
                        Some(card_document) => {
                            match card_document.get_i64("id") {
                                Ok(id) => ids_vec.push(id),
                                Err(_) => {}
                            }
                        },
                        None => {}
                    };
                }

                return Some(responses::GetUserPageInfoResponse{ username : username, card_ids : ids_vec });
            },
            None => { return None; }
        };
    }

    pub async fn get_card_next_id(&self, user_id : UserId) -> Option<responses::GetUserNextIdResponse> {
        let filter = match to_document(&user_id) {
            Some(document) => document,
            None => { return None; }
        };

        let find_options = FindOneOptions::builder().projection(doc! { "next_card_id": 1 }).build();
        let find_result = self.collection.find_one(filter, find_options).await;

        let find_document = match find_result {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Getting unsers next card id failed: {}", error);
                return None;
            }
        };

        match find_document {
            Some(data) => {
                let next_card_id = match data.get_i64("next_card_id") {
                    Ok(result) => result,
                    Err(_) => { return None; }
                    };

                return Some(responses::GetUserNextIdResponse{ next_card_id });
            },
            None => { return None; }
        };
    }

    pub async fn get_card(&self, user_id : &UserId, card_id : CardId) -> Option<responses::GetCardDataResponse> {
        let filter = doc! {
            "$and": [
                { "_id": user_id._id},
                { "cards.id": Bson::Int64(card_id) }
            ]
        };

        let find_options = FindOneOptions::builder().projection(doc! {"cards.$": 1}).build();
        let find_result = self.collection.find_one(filter, find_options).await;

        let find_document = match find_result {
            Ok(document) => document,
            Err(error) => {
                eprintln!("Getting user basic info failed: {}", error);
                return None;
            }
        };

       match find_document {
            Some(data) => {
                let card = match data.get_array("cards") {
                    Ok(result) => {
                        let first_entry = match result.first() {
                            Some(entry) => entry,
                            None => { return None; }
                        };

                        let first_entry_document = match first_entry.as_document() {
                            Some(entry_document) => entry_document,
                            None => { return None; }
                        };

                        first_entry_document
                    },
                    Err(_) => { return None; }
                };

                let device_type = match card.get_i32("device_type") {
                    Ok(result) => result,
                    Err(_) => { return None; }
                };

                let device_type = DeviceType::new(device_type as i64);

                let response = match device_type {
                    DeviceType::Empty() => {
                        responses::GetCardDataResponse{
                            card_id: card_id,
                            device_name: String::new(),
                            device_type: DeviceType::Empty()
                        }
                    },
                    DeviceType::ShockCaller(_) =>
                    {
                        let device_name = match card.get_str("device_name") {
                            Ok(result) => result.to_string(),
                            Err(_) => { return None; }
                            };

                        let shock_power = match card.get_i32("power") {
                            Ok(result) => result,
                            Err(_) => { return None; }
                        };

                        responses::GetCardDataResponse{
                            card_id: card_id,
                            device_name: device_name,
                            device_type: DeviceType::ShockCaller(Some(ShockCallerData{ power: shock_power as u8 })),
                        }
                    }
                };

                return Some(response);
            },
            None => { return None; }
        };
    }
    
    pub async fn create_card(&self, user_id : UserId) -> Option<CreateCardResponse> {
        let filter = match to_document(&user_id) {
            Some(document) => document,
            None => { return None; }
        };

        let find_options = FindOneOptions::builder().projection(doc! { "next_card_id": 1 }).build();
        let find_result = self.collection.find_one(filter.clone(), find_options).await;

        let find_document = match find_result {
            Ok(document) => document,
            Err(error) => {
                eprintln!("Getting unsers next card id failed: {}", error);
                return None;
            }
        };

        let next_card_id = match find_document {
            Some(data) => {
                let next_card_id = match data.get_i64("next_card_id") {
                    Ok(result) => result,
                    Err(_) => { return None; }
                    };

                next_card_id
            },
            None => { return None; }
        };

        let increment_card_id = doc!{"$inc": { "next_card_id" : 1 }};
        let increment_status = self.collection.update_one(filter.clone(), increment_card_id, None).await;

        let match_count = match increment_status {
            Ok(result) => result.matched_count,
            Err(_) => return None
        };

        if match_count != 1 {
            return None;
        }

        // Unify names for mongo db functions like push_status -> push_result
        let add_empty_card = doc! {"$push" : {"cards" : { "id": next_card_id, "device_type": 0 }}};
        let push_status = self.collection.update_one(filter, add_empty_card, None).await;

        let match_count = match push_status {
            Ok(result) => result.matched_count,
            Err(_) => return None
        };

        if match_count != 1 {
            return None;
        }

        Some(CreateCardResponse{ card_id: next_card_id })
    }

    pub async fn update_card(&self, user_id : UserId, card_data : &CardData) -> Result<(), DatabaseError> {
        let filter = doc! {
            "$and": [
                { "_id": user_id._id},
                { "cards.id": Bson::Int64(card_data.id) }
            ]
        };
        
        let mut update_doc = doc! {
            "cards.$.device_type" : card_data.device_type as i32,
            "cards.$.device_name" : card_data.name.clone()
        };

        match DeviceType::new(card_data.device_type) {
            DeviceType::ShockCaller(_) => {
                if let Some(mut power) = card_data.device_properties.get("power").and_then(|value| value.as_i64()) {
                    //Clamp to 1 - 10
                    power = max(1, power);
                    power = min(10, power);

                    update_doc.insert("cards.$.power", power as i32);
                }
                else {
                    update_doc.insert("cards.$.power", constants::SHOCKER_DEFAULT_POWER);
                }
            },
            _ => {}
        };

        let update_doc_set = doc! {"$set" : update_doc};
        let update_result = self.collection.update_one(filter, update_doc_set,None).await;

        if update_result.is_err() {
            return Err(DatabaseError::DatabaseCardUpdateFailed);
        }
        Ok(())
    }
    
    //db.user_data.updateOne({ username : "Wiktor" }, { $pull : { cards : { id: Long("2") }}})
    pub async fn delete_card(&self, user_id : UserId, card_id : CardId) -> Result<(), DatabaseError> {
        let is_device_type_empty = self.is_card_device_of_type(user_id.clone(), card_id, DeviceType::Empty()).await;

        match is_device_type_empty {
            Some(is_empty) => {
                if is_empty {
                    return Err(DatabaseError::CannotDeleteCardWithEmptyDeviceType);
                }
                // else continue
            },
            None => {
                return Err(DatabaseError::CheckingCardDeviceTypeFailed);
            }
        }

        let filter = match to_document(&user_id) {
            Some(document) => document,
            None => { return Err(DatabaseError::UserIdFilterCreationFailed) }
        };

        let update_document = doc! {
            "$pull" : { "cards" : { "id" : card_id } }
        };

        let update_result = self.collection.update_one(filter, update_document, None).await;

        if update_result.is_err() {
            return Err(DatabaseError::DatabaseCardUpdateFailed);
        }

        Ok(())
    }

    async fn is_card_device_of_type(&self, user_id : UserId, card_id : CardId, device_type : DeviceType) -> Option<bool> {
        let filter = doc! {
            "$and": [
                { "_id": user_id._id},
                { "cards.id": Bson::Int64(card_id) }
            ]
        };

        let find_options = FindOneOptions::builder().projection(doc! {"cards.$": 1}).build();
        let find_result = self.collection.find_one(filter, find_options).await;

        let find_document = match find_result {
            Ok(document) => document,
            Err(error) => {
                eprintln!("Getting user basic info failed: {}", error);
                return None;
            }
        };

       let find_result_document = match find_document {
            Some(document) =>  document,
            None => return None
        };

        let card_option = match find_result_document.get_array("cards") {
            Ok(result) => {
                let first_entry = match result.first() {
                    Some(entry) => entry,
                    None => { return None }
                };

                first_entry.as_document()
            },
            Err(_) => { return None }
        };

        let card = match card_option {
            Some(card) => card,
            None => { return None; }
        };

        let card_device_type = match card.get_i32("device_type") {
            Ok(result) => result,
            Err(_) => { return None; }
        };

        let card_device_type = DeviceType::new(card_device_type as i64);

        Some(card_device_type == device_type)
    }
}
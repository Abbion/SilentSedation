use mongodb::{ options::FindOneOptions, Collection, Database };
use bson::{ doc, document, Bson, Document };
use serde_json::Number;
use crate::communication::requests::CardData;
use crate::communication::{ requests, responses };
use crate::database::{ to_document, UserId };
use crate::utils::deviceTypes::{ DeviceType, ShockCallerData };
use crate::constants;

use super::CardId;

pub struct UserDataCollection {
    collection : Collection<Document>
}

impl UserDataCollection {
    pub fn new(db: &Database) -> UserDataCollection {
        UserDataCollection{ collection: db.collection::<Document>("user_data") }
    }

    pub async fn get_user_id(&self, login_data: requests::LoginUserRequest) -> Option<UserId> {
        let filter = match to_document(&login_data) {
            Some(f) => f,
            None => { return None; }
        };

        let find_options = FindOneOptions::builder().projection(doc! {"_id": 1}).build();
        let find_result = self.collection.find_one(filter, find_options).await;
    
         let fr = match find_result {
            Ok(fr) => fr,
            Err(e) => {
                eprintln!("getting user id failed: {}", e);
                return None;
            }
        };

        let user_id_result = match fr {
            Some(doc) => bson::from_bson::<UserId>(Bson::Document(doc)),
            None => { return None; }
        };

        match user_id_result {
            Ok(uir) => Some(uir),
            Err(e) => {
                eprintln!("Document to UserId conversion failed: {}", e);
                return None;
            }
        }
    }

    pub async fn get_user_page_info(&self, user_id: UserId) -> Option<responses::GetUserPageInfoResponse> {
        let filter = match to_document(&user_id) {
            Some(f) => f,
            None => { return None; }
        };

        let find_options = FindOneOptions::builder().projection(doc! {"username": 1, "cards.id": 1}).build();
        let find_result = self.collection.find_one(filter, find_options).await;

        let fr = match find_result {
            Ok(fr) => fr,
            Err(e) => {
                eprintln!("Getting user basic info failed: {}", e);
                return None;
            }
        };

        match fr {
            Some(data) => {
                let username = match data.get_str("username") {
                    Ok(res) => res.to_string(),
                    Err(_) => { return None; }
                };

                let cards = match data.get_array("cards") {
                    Ok(res) => res,
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

                return Some(responses::GetUserPageInfoResponse{ username : username, card_ids: ids_vec });
            },
            None => { return None; }
        };
    }

    pub async fn get_card_next_id(&self, user_id: UserId) -> Option<responses::GetUserNextIdResponse> {
        let filter = match to_document(&user_id) {
            Some(f) => f,
            None => { return None; }
        };

        let find_options = FindOneOptions::builder().projection(doc! { "next_card_id": 1 }).build();
        let find_result = self.collection.find_one(filter, find_options).await;

        let fr = match find_result {
            Ok(fr) => fr,
            Err(e) => {
                eprintln!("Getting unsers next card id failed: {}", e);
                return None;
            }
        };

        match fr {
            Some(data) => {
                let next_card_id = match data.get_i64("next_card_id") {
                    Ok(res) => res,
                    Err(_) => { return None; }
                    };

                return Some(responses::GetUserNextIdResponse{ next_card_id });
            },
            None => { return None; }
        };
    }

    pub async fn get_card(&self, user_id: UserId, card_id : CardId) -> Option<responses::GetCardDataResponse> {
        let filter = doc! {
            "$and": [
                { "_id": user_id._id},
                { "cards.id": Bson::Int64(card_id) }
            ]
        };

        let find_options = FindOneOptions::builder().projection(doc! {"cards.$": 1}).build();
        let find_result = self.collection.find_one(filter, find_options).await;

        let fr = match find_result {
            Ok(fr) => fr,
            Err(e) => {
                eprintln!("Getting user basic info failed: {}", e);
                return None;
            }
        };

       match fr {
            Some(data) => {
                let card = match data.get_array("cards") {
                    Ok(res) => {
                        let first_entry = match res.first() {
                            Some(entry) => entry,
                            None => { return None; }
                        };

                        let first_entry_doc = match first_entry.as_document() {
                            Some(entry_doc) => entry_doc,
                            None => { return None; }
                        };

                        first_entry_doc
                    },
                    Err(_) => { return None; }
                };

                let device_type = match card.get_i32("device_type") {
                    Ok(res) => res,
                    Err(_) => { return None; }
                };

                let device_type = DeviceType::new(device_type);

                let resp = match device_type {
                    DeviceType::EMPTY => {
                        responses::GetCardDataResponse{
                            card_id: card_id,
                            device_name: String::new(),
                            device_type: DeviceType::EMPTY,
                            code: [0, 0, 0, 0, 0, 0]
                        }
                    },
                    DeviceType::SHOCK_CALLER(_) =>
                    {
                        let device_name = match card.get_str("device_name") {
                            Ok(res) => res.to_string(),
                            Err(_) => { return None; }
                            };

                        let shock_power = match card.get_i32("power") {
                            Ok(res) => res,
                            Err(_) => { return None; }
                        };

                        let code = match card.get_str("code") {
                            Ok(res) => {
                                if res.len() != constants::DEVICE_CODE_LENGTH {
                                    return  None;
                                }

                                res.to_string()
                            },
                            Err(_) => { return None; }
                        };

                        let mut code_arr :[u8;6] = [0; 6];
                        for (index, number) in code.chars().enumerate() {
                            code_arr[index] = match number.to_digit(10) {
                                Some(num) => num as u8,
                                None => return None,
                            } 
                        }

                        responses::GetCardDataResponse{
                            card_id: card_id,
                            device_name: device_name,
                            device_type: DeviceType::SHOCK_CALLER(Some(ShockCallerData{ power: shock_power as u8 })),
                            code: code_arr
                        }
                    }
                };

                return Some(resp);
            },
            None => { return None; }
        };
    }
    
    pub async fn update_card(&self, user_id: UserId, card_data: &CardData) {
        let filter = doc! {
            "$and": [
                { "_id": user_id._id},
                { "cards.id": Bson::Int64(card_data.id) }
            ]
        };

        let update = doc! {
            "$set" : {
                "cards.$.device_type" : card_data.deviceType ,
                "cards.$.device_name" : card_data.name.clone(),
                "cards.$.power" : 2 ,
                "cards.$.code" : "2137"
             }
        };

        let update_resolt = self.collection.update_one(filter, update,None).await;
        println!("{:?}", update_resolt);
    }

pub async fn create_card(&self, user_id: UserId) {

}

pub async fn delete_card(&self, user_id: UserId, card_id : CardId) {

}
}
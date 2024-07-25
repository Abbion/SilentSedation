use std::vec;

use mongodb::{ options::FindOneOptions, Collection, Database };
use bson::{ Bson, doc , Document };
use crate::communication::{ requests, responses };
use crate::database::{ to_document, UserId };

pub struct UserDataCollection {
    collection : Collection<Document>
}

impl UserDataCollection {
    pub fn new(db: &Database) -> UserDataCollection {
        UserDataCollection{ collection: db.collection::<Document>("users_data") }
    }

    pub async fn get_user_id(&self, login_data: requests::LoginUserRequest) -> Option<UserId> {
        let filter = match to_document(&login_data) {
            Some(f) => f,
            None => return None
        };
        
        let find_options = FindOneOptions::builder().projection(doc! {"_id": 1}).build();
        let find_result = self.collection.find_one(filter, find_options).await;
    
         let fr = match find_result {
            Ok(fr) => fr,
            Err(e) => {
                eprintln!("getting user id failed: {}", e);
                None
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
                None
            }
        }
    }

    pub async fn get_user_page_info(&self, user_id: UserId) -> Option<responses::GetUserPageInfoResponse> {
        let filter = match to_document(&user_id) {
            Some(f) => f,
            None => return None
        };

        let find_options = FindOneOptions::builder().projection(doc! {"username": 1, "cards.id": 1}).build();
        let find_result = self.collection.find_one(filter, find_options).await;

        let fr = match find_result {
            Ok(fr) => fr,
            Err(e) => {
                eprintln!("Getting user basic info failed");
                None
            }
        };

        match fr {
            Some(data) => {
                let username = match data.get_str("username") {
                    Ok(res) => res.to_string(),
                    Err(_) => return None
                };

                let cards = match data.get_array("cards") {
                    Ok(res) => res,
                    Err(_) => return None
                };

                let mut ids_vec = Vec::new();

                for card in cards {
                    match card.as_document() {
                        Some(card_document) => {
                            match card_document.get_i32("id") {
                                Ok(id) => ids_vec.push(id),
                                Err(_) => {}
                            }
                        },
                        None => {}
                    };
                }

                return Some(responses::GetUserPageInfoResponse{ username : username, card_ids: ids_vec });
            },
            None => return None
        };
    }

pub async fn get_card() {

}

pub async fn update_card() {

}

pub async fn create_new_card() {

}

pub async fn delete_card() {

}
}
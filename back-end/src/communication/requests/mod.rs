use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct GetCardRequest {
    pub token : String,
    pub card_id: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBasicUserRequest {
    pub token : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUserRequest {
    pub username : String,
    pub password : String
}
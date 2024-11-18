use crate::utils::deviceTypes::DeviceType;
use serde_json::Value;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct CardData {
    pub id : i64,
    pub name : String,
    pub deviceType : i64,
    pub deviceProperties: Value,
    pub code : [u8; 6]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCardRequest {
    pub token : String,
    pub card_data : CardData
}
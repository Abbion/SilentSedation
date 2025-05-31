// Rework 3.0

use std::str;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use crate::{database::CardId, enums::device_actions::DeviceActionTypeValue, utils::device_types::DeviceTypeValue};

#[derive(Deserialize, Debug)]
pub struct GetCardRequest {
    pub token : String,
    pub card_id : CardId
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
    pub id : CardId,
    pub name : String,
    pub device_type : DeviceTypeValue,
    pub device_properties : Value,
    pub code : String
}

#[derive(Deserialize, Debug)]
pub struct CreateCardRequest {
    pub token : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectCardToDeviceRequest {
    pub token : String,
    pub id : CardId, //Change to card_id
    pub device_type : DeviceTypeValue,
    pub code : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCardRequest {
    pub token : String,
    pub card_data : CardData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PerformActionOnDeviceRequest {
    pub token : String,
    pub card_id : CardId,
    pub device_type : DeviceTypeValue,
    pub action_type : DeviceActionTypeValue
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteCardRequest {
    pub token : String,
    pub card_id : CardId
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterDeviceRequest {
    pub device_id : String,
    pub device_type : i64, // Chagne it to deviceType
}

//TODO: add request
#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateDeviceCode {
    pub device_id : String,
    pub device_type : DeviceTypeValue
}
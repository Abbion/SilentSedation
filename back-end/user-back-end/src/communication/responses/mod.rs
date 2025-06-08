// Refactor 4.0
use serde::Serialize;
use crate::{database::CardId, enums::device_type::DeviceType};

#[derive(Serialize)]
pub struct LoginResposne {
    pub token : String
}

#[derive(Serialize)]
pub enum BadRequestCodes {
    IncorrectLoginCredentails,
}

#[derive(Serialize)]
pub struct BadRequestResponse {
    pub message : String,
    pub code : BadRequestCodes
}

#[derive(Serialize, Debug)]
pub struct GetUserPageInfoResponse {
    pub username : String,
    pub card_ids : Vec<CardId>
}

#[derive(Serialize, Debug)]
pub struct GetUserNextIdResponse {
    pub next_card_id : CardId
}

#[derive(Serialize, Debug)]
pub struct GetCardDataResponse {
    pub card_id : CardId,
    pub device_name : String,
    pub device_type : DeviceType
}

#[derive(Serialize, Debug)]
pub struct CreateCardResponse {
    pub card_id : CardId
}

#[derive(Serialize)]
pub struct DeviceConnectionResponse {
    pub success : bool,
    pub  message : String,
}

#[derive(Serialize)]
pub struct DeviceRegisterResponse {
    pub new_registration : bool,
    pub message : String
}
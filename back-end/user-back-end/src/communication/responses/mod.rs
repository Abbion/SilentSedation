// Rework 3.0

use serde::Serialize;
use crate::utils::device_types::DeviceType;

#[derive(Serialize)]
pub struct LoginResposne {
    pub token : String
}

#[derive(Serialize)]
pub struct UserPageDataResponse {
    pub username : String,
    pub cards_ids : Vec<u32>
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

#[derive(Serialize)]
pub struct NextIdResponse {
    pub next_id : u64
}

#[derive(Serialize, Debug)]
pub struct GetUserPageInfoResponse {
    pub username : String,
    pub card_ids : Vec<i64> //Change to u32
}

#[derive(Serialize, Debug)]
pub struct GetUserNextIdResponse {
    pub next_card_id : i64
}

// add is active flag
#[derive(Serialize, Debug)]
pub struct GetCardDataResponse {
    pub card_id : i64,
    pub device_name : String,
    pub device_type : DeviceType,
    pub code : [u8; 6],
}

#[derive(Serialize, Debug)]
pub struct CreateCardResponse {
    pub card_id : i64
}

#[derive(Serialize)]
pub struct DeviceUpdateResponse {
    pub success: bool,
    pub  message: String,
}
use serde::Serialize;

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
    pub card_ids : Vec<i32> //Change to u32
}

#[derive(Serialize, Debug)]
pub struct GetUserNextIdResponse {
    pub next_card_id : i64
}
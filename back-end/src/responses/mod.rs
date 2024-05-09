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
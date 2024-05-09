use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetUserPageInfoRequest {
    pub token : String
}

#[derive(Deserialize, Debug)]
pub struct GetCardInfo {
    pub token : String,
    pub card_id : u64
}
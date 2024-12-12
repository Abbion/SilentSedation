//Rework 3.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ShockCallerData{
    pub power : u8
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum DeviceType{
    Empty(),
    ShockCaller(Option<ShockCallerData>)
}

impl DeviceType {
    pub fn new(type_id : i64) -> DeviceType {
        match type_id {
            1 => return DeviceType::ShockCaller(None),
            _ => return DeviceType::Empty()
        }
    }
}
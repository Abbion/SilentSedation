//Rework 3.0
//TODO: Move this to enums

use serde::{Deserialize, Serialize};

pub type DeviceTypeValue = i64;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ShockCallerData{
    pub power : u8
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum DeviceType {
    Empty(),
    ShockCaller(Option<ShockCallerData>)
}

impl DeviceType {
    pub fn new(type_id : DeviceTypeValue) -> DeviceType {
        match type_id {
            1 => return DeviceType::ShockCaller(None),
            _ => return DeviceType::Empty()
        }
    }

    pub fn as_native_value(&self) -> DeviceTypeValue {
        match self {
            DeviceType::ShockCaller(_) => 1,
            DeviceType::Empty() => 0
        }
    }
}
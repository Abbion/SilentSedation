//TODO: Move this to enums

use serde::{Deserialize, Serialize};

pub  type DeviceStateValue = i32;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum DeviceState {
    Offline,
    Online
}

impl DeviceState {
    pub fn new(type_id : DeviceStateValue) -> DeviceState {
        match type_id {
            1 => return DeviceState::Online,
            _ => return DeviceState::Offline
        }
    }

    pub fn as_native_value(&self) -> DeviceStateValue {
        match self {
            DeviceState::Online => 1,
            DeviceState::Offline => 0
        }
    }
}
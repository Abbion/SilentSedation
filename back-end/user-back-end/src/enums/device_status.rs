use serde::{Deserialize, Serialize};

pub  type DeviceStateValue = i32;

// TODO Change state to status
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum DeviceState {
    Offline,
    Online
}

impl DeviceState {
    // For future use
    #[allow(dead_code)]
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
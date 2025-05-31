use bson::DateTime;
use crate::{enums::device_actions::DeviceActionType, utils::device_types::DeviceType};

#[derive(Debug)]
pub struct DeviceEvent {
    pub time_stamp : DateTime,
    pub device_type : DeviceType,
    pub action_type : DeviceActionType
}
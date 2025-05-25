use bson::DateTime;
use crate::utils::device_types::DeviceType;

pub enum DeviceAction {
    ChangePower(i8),
    UseShock(bool)
}

pub struct DeviceEvent {
    pub time_stamp : DateTime,
    pub device_type : DeviceType,
    pub action : DeviceAction
}
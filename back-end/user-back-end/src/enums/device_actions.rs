// Refactor 4.0
pub type DeviceActionTypeValue = i64;

#[derive(PartialEq, Eq, Debug)]
pub enum DeviceActionType {
    None,
    Zap(u8)
}

impl DeviceActionType {
    pub fn new(type_id : DeviceActionTypeValue) -> DeviceActionType {
        match type_id {
            1 => return DeviceActionType::Zap(0),
            _ => return DeviceActionType::None
        }
    }

    // For future use
    #[allow(dead_code)]
    pub fn as_native_value(&self) -> DeviceActionTypeValue {
        match self {
            DeviceActionType::Zap(_) => 1,
            DeviceActionType::None => 0
        }
    }
}
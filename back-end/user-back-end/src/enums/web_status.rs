
pub type WebStatusValue = u32;

pub enum WebStatus {
    OFFLINE = 0,
    ONLINE = 1
}

impl WebStatus {
    pub fn new(status : WebStatusValue) -> WebStatus {
        match status {
            1 => return WebStatus::ONLINE,
            _ => return WebStatus::OFFLINE
        }
    }

    pub fn as_native_value(&self) -> WebStatusValue {
        match self {
            WebStatus::OFFLINE=> 0,
            WebStatus::ONLINE => 1
        }
    }
}
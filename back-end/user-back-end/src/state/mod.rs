// Rework 3.0

use std::collections::{BTreeSet, HashMap};
use tokio::sync::Mutex;
use crate::auth;
use crate::code_generator::Code;
use crate::database::DeviceId;
use crate::events::device_events::DeviceEvent;
use crate::Database;

pub struct AppState {
    pub jwt : auth::jwt::JsonWebTokenData,
    pub db : Mutex<Database>,
    pub generated_codes : Mutex<BTreeSet<Code>>,
    pub device_events : Mutex<HashMap<DeviceId, DeviceEvent>>
}
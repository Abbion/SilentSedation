// Rework 3.0

use std::collections::BTreeSet;
use tokio::sync::Mutex;
use crate::auth;
use crate::code_generator::Code;
use crate::Database;

pub struct AppState {
    pub jwt : auth::jwt::JsonWebTokenData,
    pub db : Mutex<Database>,
    pub generated_codes : Mutex<BTreeSet<Code>>
}
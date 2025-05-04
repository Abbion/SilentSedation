// Rework 3.0

use std::sync::Mutex;
use crate::auth;
use crate::Database;

pub struct AppState {
    pub jwt : auth::jwt::JsonWebTokenData,
    pub db : Mutex<Database>
}
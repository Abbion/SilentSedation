// Refactor 4.0
use std::{sync::Arc, time::Duration};
use mongodb::Database;
use crate::{code_generator::Code,
        constants::CODE_CHECK_INTERVAL_TIME_IN_SEC,
        database::{self, Collection},
        state::AppState
        };

async fn clear_old_device_codes_from_db(database : &Database) -> Option<Vec<Code>> {
    let collection = match database::get_collection(&database, database::CollectionType::DeviceCodeCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("Error: Getting code collection failed in clear_old_device_codes_from_db: {}", error);
            return  None;
        }        
    };

    let device_code_collection= match collection {
        Collection::DeviceCode(collection) => collection,
        _ => {
            eprintln!("clear_old_device_codes_from_db failed to acquire the device code collection");
            return None;
        }
    };

    device_code_collection.remove_device_expired_codes().await
}

pub async fn clear_old_device_codes(app_state : Arc<AppState>) {
    let sleep_duration = Duration::from_secs(CODE_CHECK_INTERVAL_TIME_IN_SEC);

    loop {
        {
            let acquired_db = app_state.db.lock().await;
            let acquired_codes = clear_old_device_codes_from_db(&acquired_db).await;

            if let Some(codes) = acquired_codes {
                let mut codes_cache = app_state.generated_codes.lock().await;
                
                for code in codes {
                    codes_cache.remove(&code);
                }
            }
        }

        tokio::time::sleep(sleep_duration).await;
    }
}
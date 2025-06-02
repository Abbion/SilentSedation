use std::{sync::Arc, time::Duration};
use chrono::Utc;
use crate::{constants::{EVENT_CHECK_INTERVAL_TIME_IN_MIN, EVENT_EXPIRATION_TIME_IN_MIN}, state::AppState};


pub async fn clear_old_device_events_from_queue(app_state : Arc<AppState>) {
    let sleep_duration = Duration::from_secs(EVENT_CHECK_INTERVAL_TIME_IN_MIN * 60);

    loop {
        {
            let mut events = app_state.device_events.lock().await;
            let current_time = Utc::now();

            events.retain(|_, event|{
                current_time.signed_duration_since(event.time_stamp.to_chrono()) <
                chrono::Duration::minutes(EVENT_EXPIRATION_TIME_IN_MIN)
            });
        }

        tokio::time::sleep(sleep_duration).await;
    }
}
use std::{sync::Arc, time::Duration};
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines}, net::tcp::{OwnedReadHalf, OwnedWriteHalf}, time::timeout};
use crate::{communication::requests::GenerateDeviceCode,
            constants::{DEVICE_NEXT_LINE_AWAIT_TIME_IN_MS, EVENT_LOOP_INTERVAL_TIME_IN_MS, MAX_DEVICE_DATA_REQUERT_ATTEMPTS},
            database::{self, Collection, DeviceId},
            enums::{device_actions::DeviceActionType, web_status::WebStatus},
            events::device_events::DeviceEvent,
            state::AppState,
            utils::device_types::DeviceType };

pub async fn handle_device_connection(stream : tokio::net::TcpStream, app_state : Arc<AppState>) {
    println!("Device connected: {:?}", stream);

    let (device_data, mut lines, mut write_half) = get_device_data(stream).await;

    let device_data = match device_data {
        Some(data) => data,
        None => {
            eprintln!("No device data found for socket!");
            return;
        }
    };

    let device_id = match DeviceId::from_str(&device_data.device_id) {
        Some(id) => id,
        None => {
            eprintln!("Device Id can't be parsed for socket");
            return;
        }
    };

    let device_type = DeviceType::new(device_data.device_type);

    if device_type == DeviceType::Empty() {
        eprintln!("Device type can't be parsed for socket");
        return;
    }

    if update_device_web_status(&device_id, WebStatus::ONLINE, app_state.clone()).await == false {
        eprintln!("Failed to update device web status as online");
        return;
    }

    let sleep_duration = Duration::from_millis(EVENT_LOOP_INTERVAL_TIME_IN_MS);
    let device_await_duration = Duration::from_millis(DEVICE_NEXT_LINE_AWAIT_TIME_IN_MS);

    loop {
        tokio::time::sleep(sleep_duration).await;

        match timeout(device_await_duration, lines.next_line()).await {
            Ok(Ok(Some(line))) => {
                if line == "exit" {
                    break;
                }
            },
            Ok(Ok(None)) => {
                break;
            },
            Ok(Err(error)) => {
                eprintln!("Error while reading lines in device socket: {}", error)
            },
            Err(_) => {
                //Timeout
            }
        }

        {
            let mut device_events = app_state.device_events.lock().await;
            let event = match device_events.get(&device_id) {
                Some(event) => event,
                None => {
                    continue;
                }
            };

            let response = get_response_for_event(event, &device_type);
            device_events.remove(&device_id);

            println!("Response: {:?}", response);

            match response {
                Some(response) => {
                    if let Err(error) = write_half.write_all(response.as_bytes()).await {
                        eprintln!("Can't to write to device: {}. \nCloseing connection", error);
                        break;
                    }
                }
                None => { 
                    eprintln!("Failed to get the response for device on socket");
                }
            }
        }
    }

    if update_device_web_status(&device_id, WebStatus::OFFLINE, app_state).await == false {
        eprintln!("Failed to update device web status as offline");
        return;
    }

    println!("Device disconnected");
}

async fn update_device_web_status(device_id : &DeviceId, web_status : WebStatus, app_state : Arc<AppState>) -> bool {
    let db = app_state.db.lock().await;

    let collection = match database::get_collection(&db, database::CollectionType::DeviceCollection).await {
        Ok(device_collection) => device_collection,
        Err(error) => {
            eprintln!("Failed to update device web status: 1 {}", error);
            return false;
        }
    };

    let device_collection= match collection {
        Collection::Device(device) => device,
        _ => {
            return false;
        }
    };

    device_collection.update_device_status(device_id, web_status).await
}

async fn get_device_data(stream : tokio::net::TcpStream) -> (Option<GenerateDeviceCode>, Lines<BufReader<OwnedReadHalf>>, OwnedWriteHalf) {
    let mut request_attempts = 0;

    let (read_half, mut write_half) = stream.into_split();
    let reader = BufReader::new(read_half);
    let mut lines = reader.lines();

    loop {
        if request_attempts >= MAX_DEVICE_DATA_REQUERT_ATTEMPTS {
            eprintln!("Device attempted to send device data but failed");
            break;
        }

        let device_data = match lines.next_line().await {
            Ok(result) => {
                if let Some(result_str) = result {
                    result_str
                }
                else {
                    eprintln!("Geting device data failed while reading 1");
                    request_attempts += 1;
                    continue;
                }
            },
            Err(error) =>{
                eprintln!("Geting device data failed while reading 2: {}", error);
                request_attempts += 1;
                continue;
            }
        };

        let device_data : GenerateDeviceCode = match serde_json::from_str(&device_data) {
          Ok(parsed_data)  => parsed_data,
          Err(error) => {
            eprintln!("Geting device data failed while parsing json: {}", error);
            request_attempts += 1;
            continue;
          }
        };

        let response = format!("ACK: {}\n", "success");

        if let Err(error) = write_half.write_all(response.as_bytes()).await {
            eprintln!("Geting device data failed while responding: {}", error);
        } else {
            return (Some(device_data), lines, write_half);
        }
    }

    (None, lines, write_half)
}

fn get_response_for_event(event: &DeviceEvent, device_type: &DeviceType) -> Option<String> {
    if event.device_type.as_native_value() != device_type.as_native_value() {
        return None;
    }

    match event.action_type {
        DeviceActionType::Zap(power) => {
            return Some(format!("zap: {}", power));
        },
        DeviceActionType::None => {
            return None;
        }
    }
}
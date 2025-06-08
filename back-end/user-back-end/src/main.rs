// Refactor 4.0
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{ web, App, HttpServer};
use mongodb::Database;
use private::PrivateKeys;
use state::AppState;
use tokio::{ sync::Mutex, net::TcpListener };
use device_handing::device_connection::handle_device_connection;
use utils::cleaners;
use crate::database::{get_collection, Collection};

mod constants;
mod enums;
mod auth;
mod private;
mod communication;
mod database;
mod services;
mod utils;
mod state;
mod code_generator;
mod events;
mod device_handing;

const SERVER_ADDRESS : &str = "127.0.0.1";
const DEVICE_PORT : &str = "9010";
const SERVICE_PORT : u16 = 9000;

async fn initialize() -> (Database, PrivateKeys) {
    let db_handle = tokio::spawn(async {
        database::connect_to_database().await
    });
    let private_keys_handle = tokio::spawn(async {
        private::get_private_keys().await
    });

    let (db_joint, private_keys_join) = tokio::join!(db_handle, private_keys_handle);

    let private_keys = match private_keys_join {
        Ok(private_keys) => private_keys,
        Err(error) => {
            panic!("private keys join failed: {}", error);
        }
    };

    let db = match db_joint {
        Ok(db) => db,
        Err(error) =>{
            panic!("db joint failed: {}", error);
        }
    };

    let collection = match get_collection(&db, database::CollectionType::DeviceCollection).await {
        Ok(collection) => collection,
        Err(error) => {
            panic!("Failed to get the device collection while Init: {}", error);
        }
    };

    let device_collection= match collection {
        Collection::Device(device) => device,
        _ => {
            panic!("Failed get the device collection while Init");
        }
    };

    if device_collection.put_all_devices_offline().await == false {
        panic!("Failed to put all devices as offline while Init");
    }

    (db, private_keys)
}

async fn start_socket_server(app_state : Arc<AppState>) -> std::io::Result<()> {
    let adr = std::format!("{}:{}", SERVER_ADDRESS, DEVICE_PORT);
    let listener = TcpListener::bind(adr).await?;
    println!("Socket server listening on port 9010");

    loop {
        let (socket, _) = listener.accept().await?;
        let device_app_state = app_state.clone();
    
        tokio::spawn(async move {
            handle_device_connection(socket, device_app_state).await;
        });
    }
}

#[actix_web::main]
async fn main() {
    let (db, private_keys) =  initialize().await;

    let app_state = Arc::new(state::AppState{
        jwt : auth::jwt::JsonWebTokenData::new(&private_keys),
        db : Mutex::new(db),
        generated_codes : Mutex::new(BTreeSet::new()),
        device_events : Mutex::new(HashMap::new())
    });

    let socket_server_app_state = app_state.clone();
    tokio::spawn(async move {
        start_socket_server(socket_server_app_state).await.unwrap();
    });

    let clear_codes_task_app_state = app_state.clone();
    tokio::spawn(async move {
        cleaners::code_clean::clear_old_device_codes(clear_codes_task_app_state).await;
    });

    let clear_events_task_app_state = app_state.clone();
    tokio::spawn(async move {
        cleaners::event_clean::clear_old_device_events_from_queue(clear_events_task_app_state).await;
    });

    let web_server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allow_any_header()
                    .max_age(3600)
            )
        .app_data(web::Data::new(app_state.clone()))
        .service(services::login_user)
        .service(services::get_user_page_info)
        .service(services::get_next_card_id)
        .service(services::get_card)
        .service(services::create_card)
        .service(services::update_card)
        .service(services::delete_card)
        .service(services::get_card_states)
        .service(services::connect_card_to_device)
        .service(services::perform_action_on_device)
        //Device calls
        .service(services::register_device)
        .service(services::generate_device_code)
    })
    .workers(4)
    .bind((SERVER_ADDRESS, SERVICE_PORT));

    let running_server = match web_server {
        Ok(server) => {
            println!("Web server has started on port: 9000");
            server.run()
        }
        Err(error) => {
            eprintln!("Starting the web server failed: {}", error);
            return;
        }
    };

    match running_server.await {
        Ok(_) => {
            println!("Web server exited with OK");
        },
        Err(error) => {
            eprintln!("The web server exited with an error: {}", error);
        }
    }
}
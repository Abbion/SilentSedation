// Rework 3.0

use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;
use std::time::Duration;
use actix_cors::Cors;
use actix_web::{ web, App, HttpServer };
use code_generator::Code;
use constants::CODE_CHECK_INTERVAL_TIME_IN_SEC;
use database::Collection;
use mongodb::Database;
use private::PrivateKeys;
use state::AppState;
use tokio::{ sync::Mutex, net::TcpListener };
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

mod constants;
mod auth;
mod private;
mod communication;
mod database;
mod services;
mod utils;
mod state;
mod code_generator;
mod events;

async fn initialize() -> (Database, PrivateKeys) {
    let db_handle = tokio::spawn(async {
        database::connect_to_database().await
    });
    let private_keys_handle = tokio::spawn(async {
        private::get_private_keys().await
    });

    let (db_joint, private_keys_join) = tokio::join!(db_handle, private_keys_handle);

    let db = match db_joint {
        Ok(db) => db,
        Err(error) =>{
            panic!("db joint failed: {}", error);
        }
    };

    let private_keys = match private_keys_join {
        Ok(private_keys) => private_keys,
        Err(error) => {
            panic!("private keys join failed: {}", error);
        }
    };

    (db, private_keys)
}

async fn handle_device_connection(stream : tokio::net::TcpStream) {
    println!("Device connected: {:?}", stream);

    let (read_half, mut write_half) = stream.into_split();
    let reader = BufReader::new(read_half);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        println!("Socket received: {}", line);

        let response = format!("ACK: {}\n", line);
        if let Err(e) = write_half.write_all(response.as_bytes()).await {
            eprintln!("Failed to write to arduino: {}", e);
            break;
        }
    }

    println!("Device disconnected");
}

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

async fn clear_old_device_codes(app_state : Arc<AppState>) {
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

async fn start_socket_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9010").await?;
    println!("Socket server listening on port 9010");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_device_connection(socket).await;
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

    tokio::spawn(async move {
        start_socket_server().await.unwrap();
    });

    let clear_task_app_state = app_state.clone();
    tokio::spawn(async move {
        clear_old_device_codes(clear_task_app_state).await;
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
        //Device calls
        .service(services::register_device)
        .service(services::generate_device_code)
    })
    .workers(4)
    .bind(("127.0.0.1", 9000));

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
// Rework 3.0

use std::io::Lines;
use std::sync::{Arc, Mutex};
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer };
use mongodb::Database;
use private::PrivateKeys;
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

mod constants;
mod auth;
mod private;
mod communication;
mod database;
mod services;
mod utils;
mod state;

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
async fn main() -> std::io::Result<()> {
    let (db, private_keys) =  initialize().await;

    let web_data = Data::new(state::AppState{
        jwt : auth::jwt::JsonWebTokenData::new(&private_keys),
        db : Mutex::new(db)
    });

    tokio::spawn(async move {
        start_socket_server().await.unwrap();
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allow_any_header()
                    .max_age(3600)
            )
        .app_data(web_data.clone())
        .service(services::login_user)
        .service(services::get_user_page_info)
        .service(services::get_next_card_id)
        .service(services::get_card)
        .service(services::create_card)
        .service(services::update_card)
        .service(services::delete_card)
        //Device calls
        .service(services::register_device)
        //.service(services::generate_code_for_device)
        //.service(services::connect_device_by_code)
    })
    .workers(4)
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}
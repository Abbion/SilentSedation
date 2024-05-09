use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_web::{get, http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder};
use database::LoginData;
use mongodb::Database;
use private::PrivateKeys;
use serde::{ Deserialize, Serialize };
use tokio::sync::futures;

mod auth;
mod private;
mod responses;
mod requests;
mod database;

struct AppState {
    jwt : auth::jwt::JsonWebTokenData,
    next_card_id : Arc<Mutex<u64>>
}

#[get("/test")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Wiktor!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("Hello ({})", req_body);
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn path_extractor(path: web::Path<(u32, String)>) -> impl Responder {
    let (user_id, friend) = path.into_inner();
    HttpResponse::Ok().body(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[post("/login")]
async fn login_user(body: web::Json<LoginData>, data: web::Data<AppState>) -> impl Responder {
    let username = &body.username;
    let password = &body.password;
    
    if username == "Wiktor" && password == "123" {
        let mut user_token = auth::jwt::UserToken::new(String::from("Wiktor"), 1);
        user_token.set_exp_in_days(30);
        
        let encoded_token = data.jwt.encode(&user_token);
        let encoded_token = match encoded_token {
            Ok(token) => token,
            Err(e) => {
                println!("{}", e);
                return HttpResponse::InternalServerError().body("Internal login error: 1");
            }
        };

        let response = responses::LoginResposne{ token : encoded_token };
        let serialized_response = serde_json::to_string(&response);
        let serialized_response = match serialized_response {
            Ok(response) => response,
            Err(e) => {
                println!("{}", e);
                return HttpResponse::InternalServerError().body("Internal login error: 2");
            }
        };

        return HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response);
    }

    let response = responses::BadRequestResponse{ message : String::from("incorrect username or password"), code : responses::BadRequestCodes::IncorrectLoginCredentails };
    let serialized_error_response = serde_json::to_string(&response);
    let serialized_error_response = match serialized_error_response {
        Ok(response) => response,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::InternalServerError().body("Internal login error: 3");
        }
    };

    HttpResponse::Unauthorized().body(serialized_error_response)
}

#[post("/get_user_page_info")]
async fn get_user_page_info(body: web::Json<requests::GetUserPageInfoRequest>, data: web::Data<AppState>) -> impl Responder {
    let token = &body.token;
    
    let user_token_result = data.jwt.decode::<auth::jwt::UserToken>(token.to_owned());

    let user_token = match user_token_result {
        Ok(user_token) => user_token.claims,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().body("User token dedode failed");
        }
    };

    let user_page_response = responses::UserPageDataResponse {
        username : String::from("Wiktor"),
        cards_ids : vec![]
    };

    let serialized_response = serde_json::to_string(&user_page_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().body("Internal login error");
        }
        
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/get_new_card_id")]
async fn get_new_card_id(body: web::Json<requests::GetUserPageInfoRequest>, data: web::Data<AppState>) -> impl Responder {
    let token = &body.token;
    
    let user_token_result = data.jwt.decode::<auth::jwt::UserToken>(token.to_owned());

    let user_token = match user_token_result {
        Ok(user_token) => user_token.claims,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().body("User token dedode failed");
        }
    };

    let mut next_id_mutex = data.next_card_id.lock().unwrap();

    let next_id_respose = responses::NextIdResponse {
        next_id : *next_id_mutex
    };

    println!("{}", next_id_mutex);
    *next_id_mutex += 1;

    let serialized_response = serde_json::to_string(&next_id_respose);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().body("Internal login error");
        }
        
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

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
        Err(e) =>{
            panic!("db joint failed: {}", e);
        }
    };

    let private_keys = match private_keys_join {
        Ok(private_keys) => private_keys,
        Err(e) => {
            panic!("private keys join failed: {}", e);
        }
    };

    (db, private_keys)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (db, private_keys) =  initialize().await;

    database::list_collections(&db).await ;
    let res = database::get_uesr_id(&db, LoginData{ username: "Wiktor".to_string(), password : "123".to_string() }).await;
    println!("User state: {:?}", res);

    let web_data = web::Data::new(AppState{
        jwt : auth::jwt::JsonWebTokenData::new(&private_keys),
        next_card_id : Arc::new(Mutex::new(1))
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
        .service(login_user)
        .service(get_user_page_info)
        .service(get_new_card_id)
        .route("/hey", web::get().to(manual_hello))
    })
    .workers(4)
    .bind(("127.0.0.1", 90))?
    .run()
    .await
}
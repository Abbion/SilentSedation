//Rework 2.0
use std::sync::{Mutex, MutexGuard};

use actix_cors::Cors;
use actix_web::{get, http::header::ContentType, post, web::{ self }, App, HttpResponse, HttpServer, Responder};
use communication::{ requests, responses };
use database::UserId;
use mongodb::Database;
use private::PrivateKeys;
use database::error_types::DatabaseError;

mod constants;
mod auth;
mod private;
mod communication;
mod database;
mod utils;

struct AppState {
    jwt : auth::jwt::JsonWebTokenData,
    db : Mutex<Database>
}

#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn path_extractor(path: web::Path<(u32, String)>) -> impl Responder {
    let (user_id, friend) = path.into_inner();
    HttpResponse::Ok().body(format!("Welcome {}, user_id {}!", friend, user_id))
}

fn lock_database<'a>(db: &'a Mutex<Database>, error_message: &str) -> Result<MutexGuard<'a, Database>, HttpResponse> {
    match db.lock() {
        Ok(db) => Ok(db),
        Err(e) => {
            eprint!("{}", e);
            Err(HttpResponse::InternalServerError().body(format!("Internal error: database lock - {}", error_message)))
        }
    }
}

#[post("/login")]
async fn login_user(body: web::Json<requests::LoginUserRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "login error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let username = body.username.to_string();
    let password = body.password.to_string();

    let user_data_collection = database::get_collections(&db).await;
    let res = user_data_collection.get_user_id(requests::LoginUserRequest{ username : username.clone(), password }).await;

    let user_id = match res {
        Some(uesr_id) => uesr_id,
        None => {
            let response = responses::BadRequestResponse{ message : String::from("incorrect username or password"), code : responses::BadRequestCodes::IncorrectLoginCredentails };
            let serialized_error_response = serde_json::to_string(&response);
            let serialized_error_response = match serialized_error_response {
            Ok(response) => response,
                Err(e) => {
                    eprint!("{}", e);
                    return HttpResponse::InternalServerError().body("Internal login error: 2");
                }
            };

            return HttpResponse::Unauthorized().body(serialized_error_response)
        }
    };

    let mut user_token = auth::jwt::UserToken::new(String::from(format!("{}", user_id)), 1);
    user_token.set_exp_in_days(30);
        
    let encoded_token = data.jwt.encode(&user_token);
    let encoded_token = match encoded_token {
        Ok(token) => token,
        Err(e) => {
            eprintln!("{}", e);
            return HttpResponse::InternalServerError().body("Internal login error: 3");
        }
    };

    let response = responses::LoginResposne{ token : encoded_token };
    let serialized_response = serde_json::to_string(&response);
    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(e) => {
            eprintln!("{}", e);
            return HttpResponse::InternalServerError().body("Internal login error: 4");
        }
    };
    
    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/get_user_page_info")]
async fn get_user_page_info(body: web::Json<requests::GetBasicUserRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "get user page info error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let token = &body.token;
    let user_token_result = data.jwt.decode::<auth::jwt::UserToken>(token.to_owned());

    let user_token = match user_token_result {
        Ok(user_token) => user_token.claims,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().body("User token dedode failed");
        }
    };

    let user_id = match UserId::from_str(&user_token.sub) {
        Some(id) => id,
        None => {
            return HttpResponse::InternalServerError().body("Internal user page info error: 2");
        }
    };

    let user_data_collection = database::get_collections(&db).await;
    let res = user_data_collection.get_user_page_info(user_id).await;

    let user_page_response = match res {
        Some(a) => a,
        None => {
            return HttpResponse::InternalServerError().body("Internal user page info error: 3");
        }
    };

    let serialized_response = serde_json::to_string(&user_page_response);

    println!("user page info: {:?}", serialized_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().body("Internal user page info error: 4");
        }
        
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/get_next_card_id")]
async fn get_next_card_id(body: web::Json<requests::GetBasicUserRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "get next id error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let token = &body.token;
    let user_token_result = data.jwt.decode::<auth::jwt::UserToken>(token.to_owned());

    let user_token = match user_token_result {
        Ok(user_token) => user_token.claims,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().body("User token dedode failed");
        }
    };

    let user_id = match UserId::from_str(&user_token.sub) {
        Some(id) => id,
        None => {
            return HttpResponse::InternalServerError().body("Internal get next id error: 2");
        }
    };

    let user_data_collection = database::get_collections(&db).await;
    let res = user_data_collection.get_card_next_id(user_id).await;

    let next_card_id_response = match res {
        Some(response) => response,
        None => {
            return HttpResponse::InternalServerError().body("Internal get next id error: 3");
        }
    };

    let serialized_response = serde_json::to_string(&next_card_id_response);

    println!("next id response {:?}", serialized_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().body("Internal get next id error: 4");
        }
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/get_card")]
async fn get_card(body: web::Json<requests::GetCardRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "get card error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let token = &body.token;
    let user_token_result = data.jwt.decode::<auth::jwt::UserToken>(token.to_owned());

    let user_token = match user_token_result {
        Ok(user_token) => user_token.claims,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().body("User token dedode failed");
        }
    };

    let user_id = match UserId::from_str(&user_token.sub) {
        Some(id) => id,
        None => {
            return HttpResponse::InternalServerError().body("Internal get card error: 2");
        }
    };

    let card_id = body.card_id;

    let user_data_collection = database::get_collections(&db).await;
    let res = user_data_collection.get_card(user_id, card_id).await;

    let get_card_response = match res {
        Some(response) => response,
        None => {
            return HttpResponse::InternalServerError().body("Internal get card error: 3");
        }
    };

    let serialized_response = serde_json::to_string(&get_card_response);

    println!("get card data: {:?}", serialized_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().body("Internal get card error: 4");
        }
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/create_card")]
async fn create_card(body: web::Json<requests::CreateCardRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "create card error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let token = &body.token;
    let user_token_result = data.jwt.decode::<auth::jwt::UserToken>(token.to_owned());

    let user_token = match user_token_result {
        Ok(user_token) => user_token.claims,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().body("User token dedode failed");
        }
    };

    let user_id = match UserId::from_str(&user_token.sub) {
        Some(id) => id,
        None => {
            return HttpResponse::InternalServerError().body("Internal create card error: 2");
        }
    };

    let user_data_collection = database::get_collections(&db).await;
    let create_card_result = user_data_collection.create_card(user_id).await;

    let create_card_response = match create_card_result {
        Some(response) => response,
        None => {
           return HttpResponse::InternalServerError().body("Internal create card error: 3");
        }     
    };

    let serialized_response = serde_json::to_string(&create_card_response);

    println!("create card data response: {:?}", serialized_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().body("Internal create card error: 4");
        }
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/update_card")]
async fn update_card(body: web::Json<requests::UpdateCardRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "update card error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let token = &body.token;
    let user_token_result = data.jwt.decode::<auth::jwt::UserToken>(token.to_owned());

    let user_token = match user_token_result {
        Ok(user_token) => user_token.claims,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().body("User token dedode failed");
        }
    };

    let user_id = match UserId::from_str(&user_token.sub) {
        Some(id) => id,
        None => {
            return HttpResponse::InternalServerError().body("Internal get card error: 2");
        }
    };

    let card_data = &body.card_data;

    println!("update card data {:?}", card_data);

    let user_data_collection = database::get_collections(&db).await;
    let card_update_result = user_data_collection.update_card(user_id, card_data).await;

    match card_update_result {
        Ok(_) => {
            HttpResponse::Ok().content_type(ContentType::json()).body("Card updated successfuly")
        },
        Err(e) => {
            match e {
                DatabaseError::CodeParsingFailed => {
                    HttpResponse::InternalServerError().body("Internal update card error: 3")
                },
                DatabaseError::DatabaseCardUpdateFailed => {
                    HttpResponse::InternalServerError().body("Internal update card error: 4")
                }
                _ => {
                    HttpResponse::InternalServerError().body("Internal update card error: X")
                }
            }
        }
    }
}

#[post("/delete_card")]
async fn delete_card(body: web::Json<requests::DeleteCardRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "delete card error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let token = &body.token;
    let user_token_result = data.jwt.decode::<auth::jwt::UserToken>(token.to_owned());

    let user_token = match user_token_result {
        Ok(user_token) => user_token.claims,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().body("User token dedode failed");
        }
    };

    let user_id = match UserId::from_str(&user_token.sub) {
        Some(id) => id,
        None => {
            return HttpResponse::InternalServerError().body("Internal delete card error: 2");
        }
    };

    let card_id = body.card_id;

    let user_data_collection = database::get_collections(&db).await;
    let card_delete_result = user_data_collection.delete_card(user_id, card_id).await;

    match card_delete_result {
             Ok(_) => {
            HttpResponse::Ok().content_type(ContentType::json()).body("Card deleted successfuly")
        },
        Err(e) => {
            match e {
                DatabaseError::CannotDeleteCardWithEmptyDeviceType => {
                    HttpResponse::InternalServerError().body("Internal delete card error: 3")
                },
                DatabaseError::CheckingCardDeviceTypeFailed => {
                    HttpResponse::InternalServerError().body("Internal delete card error: 4")
                },
                DatabaseError::UserIdFilterCreationFailed => {
                    HttpResponse::InternalServerError().body("Internal delete card error: 5")
                },
                DatabaseError::DatabaseCardUpdateFailed => {
                    HttpResponse::InternalServerError().body("Internal delete card error: 6")
                },
                _ => {
                    HttpResponse::InternalServerError().body("Internal update card error: X")
                }
            }
        }   
    }
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

    let web_data = web::Data::new(AppState{
        jwt : auth::jwt::JsonWebTokenData::new(&private_keys),
        db : Mutex::new(db)
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
        .service(get_next_card_id)
        .service(get_card)
        .service(create_card)
        .service(update_card)
        .service(delete_card)
    })
    .workers(4)
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}
// Rework 3.0

use std::sync::{Mutex, MutexGuard};
use actix_web::{http::header::ContentType, post, web::{ self }, HttpResponse, Responder};
use mongodb::Database;
use crate::{communication::{ requests, responses }, constants, database::DatabaseObjectId};
use crate::database::UserId;
use crate::database::error_types::DatabaseError;
use crate::auth;
use crate::database;
use crate::state::AppState;

fn lock_database<'a>(db: &'a Mutex<Database>, error_message: &str) -> Result<MutexGuard<'a, Database>, HttpResponse> {
    match db.lock() {
        Ok(db) => Ok(db),
        Err(error) => {
            eprintln!("{}", error);
            Err(HttpResponse::InternalServerError().body(format!("Internal error: database lock - {}", error_message)))
        }
    }
}

fn get_user_id(token: &String, jwt_decoder: &auth::jwt::JsonWebTokenData, function_name: &str) -> Result<DatabaseObjectId, HttpResponse> {
    let user_token_result = jwt_decoder.decode::<auth::jwt::UserToken>(token.to_owned());

    let user_token = match user_token_result {
        Ok(user_token) => user_token.claims,
        Err(error) => {
            eprintln!("{}", error);
            return Err(HttpResponse::Unauthorized().body(format!("User token dedode failed for {}", function_name)));
        }
    };

    match UserId::from_str(&user_token.sub) {
        Some(id) => {
            return Ok(id);
        },
        None => {
            return Err(HttpResponse::InternalServerError().body(format!("User token conversion failed for {}", function_name)));
        }
    };
}

#[post("/login")]
pub async fn login_user(body: web::Json<requests::LoginUserRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "login error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let username = body.username.to_string();
    let password = body.password.to_string();

    let user_data_collection = database::get_collections(&db).await;
    let result = user_data_collection.get_user_id(requests::LoginUserRequest{ username : username.clone(), password }).await;

    let user_id = match result {
        Some(uesr_id) => uesr_id,
        None => {
            let response = responses::BadRequestResponse{ message : String::from("incorrect username or password"), code : responses::BadRequestCodes::IncorrectLoginCredentails };
            let serialized_error_response = serde_json::to_string(&response);
            let serialized_error_response = match serialized_error_response {
            Ok(response) => response,
                Err(error) => {
                    eprintln!("{}", error);
                    return HttpResponse::InternalServerError().body("Internal login error: 2");
                }
            };

            return HttpResponse::Unauthorized().body(serialized_error_response)
        }
    };

    let mut user_token = auth::jwt::UserToken::new(String::from(format!("{}", user_id)), 1);
    user_token.set_exp_in_days(constants::TOKEN_EXPIRATION_DAYS);
        
    let encoded_token = data.jwt.encode(&user_token);
    let encoded_token = match encoded_token {
        Ok(token) => token,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal login error: 3");
        }
    };

    let response = responses::LoginResposne{ token : encoded_token };
    let serialized_response = serde_json::to_string(&response);
    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal login error: 4");
        }
    };
    
    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/get_user_page_info")]
pub async fn get_user_page_info(body: web::Json<requests::GetBasicUserRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "get user page info error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let user_data_collection = database::get_collections(&db).await;
    let result = user_data_collection.get_user_page_info(user_id).await;

    let user_page_response = match result {
        Some(response) => response,
        None => {
            return HttpResponse::InternalServerError().body("Internal user page info error: 2");
        }
    };

    let serialized_response = serde_json::to_string(&user_page_response);

    //println!("get user page info response {:?}", serialized_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{:?}", error);
            return HttpResponse::InternalServerError().body("Internal user page info error: 3");
        }
        
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/get_next_card_id")]
pub async fn get_next_card_id(body: web::Json<requests::GetBasicUserRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "get next id error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let user_data_collection = database::get_collections(&db).await;
    let result = user_data_collection.get_card_next_id(user_id).await;

    let next_card_id_response = match result {
        Some(response) => response,
        None => {
            return HttpResponse::InternalServerError().body("Internal get next id error: 2");
        }
    };

    let serialized_response = serde_json::to_string(&next_card_id_response);

    //println!("next id response {:?}", serialized_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{:?}", error);
            return HttpResponse::InternalServerError().body("Internal get next id error: 3");
        }
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/get_card")]
pub async fn get_card(body: web::Json<requests::GetCardRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "get card error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let card_id = body.card_id;

    let user_data_collection = database::get_collections(&db).await;
    let result = user_data_collection.get_card(user_id, card_id).await;

    let get_card_response = match result {
        Some(response) => response,
        None => {
            return HttpResponse::InternalServerError().body("Internal get card error: 2");
        }
    };

    let serialized_response = serde_json::to_string(&get_card_response);

    //println!("get card data: {:?}", serialized_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{:?}", error);
            return HttpResponse::InternalServerError().body("Internal get card error: 3");
        }
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/create_card")]
pub async fn create_card(body: web::Json<requests::CreateCardRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "create card error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let user_data_collection = database::get_collections(&db).await;
    let result = user_data_collection.create_card(user_id).await;

    let create_card_response = match result {
        Some(response) => response,
        None => {
           return HttpResponse::InternalServerError().body("Internal create card error: 2");
        }     
    };

    let serialized_response = serde_json::to_string(&create_card_response);

    //println!("create card data response: {:?}", serialized_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{:?}", error);
            return HttpResponse::InternalServerError().body("Internal create card error: 3");
        }
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/update_card")]
pub async fn update_card(body: web::Json<requests::UpdateCardRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "update card error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let card_data = &body.card_data;

    //println!("update card data {:?}", card_data);

    let user_data_collection = database::get_collections(&db).await;
    let result = user_data_collection.update_card(user_id, card_data).await;

    match result {
        Ok(_) => {
            HttpResponse::Ok().content_type(ContentType::json()).body("Card updated successfuly")
        },
        Err(error) => {
            match error {
                DatabaseError::CodeParsingFailed => {
                    HttpResponse::InternalServerError().body("Internal update card error: 2")
                },
                DatabaseError::DatabaseCardUpdateFailed => {
                    HttpResponse::InternalServerError().body("Internal update card error: 3")
                }
                _ => {
                    HttpResponse::InternalServerError().body("Internal update card error: X")
                }
            }
        }
    }
}

#[post("/delete_card")]
pub async fn delete_card(body: web::Json<requests::DeleteCardRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match lock_database(&data.db, "delete card error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let card_id = body.card_id;

    let user_data_collection = database::get_collections(&db).await;
    let result = user_data_collection.delete_card(user_id, card_id).await;

    match result {
             Ok(_) => {
            HttpResponse::Ok().content_type(ContentType::json()).body("Card deleted successfuly")
        },
        Err(error) => {
            match error {
                DatabaseError::CannotDeleteCardWithEmptyDeviceType => {
                    HttpResponse::InternalServerError().body("Internal delete card error: 2")
                },
                DatabaseError::CheckingCardDeviceTypeFailed => {
                    HttpResponse::InternalServerError().body("Internal delete card error: 3")
                },
                DatabaseError::UserIdFilterCreationFailed => {
                    HttpResponse::InternalServerError().body("Internal delete card error: 4")
                },
                DatabaseError::DatabaseCardUpdateFailed => {
                    HttpResponse::InternalServerError().body("Internal delete card error: 5")
                },
                _ => {
                    HttpResponse::InternalServerError().body("Internal update card error: X")
                }
            }
        }   
    }
}
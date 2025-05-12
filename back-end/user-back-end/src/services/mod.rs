// Rework 3.0

use std::sync::{Arc, Mutex, MutexGuard};
use actix_web::{http::header::ContentType, post, web::{ self }, HttpResponse, Responder};
use mongodb::Database;
use crate::{code_generator::Code, communication::{ requests, responses }, constants::{self, MAX_SHUFFLE_COUNT}, database::{ DatabaseObjectId, DeviceId }};
use crate::database::{ self, CollectionType, UserId, Collection };
use crate::database::error_types::DatabaseError;
use crate::auth;
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
pub async fn login_user(body: web::Json<requests::LoginUserRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = match lock_database(&data.db, "Internal login error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let username = body.username.to_string();
    let password = body.password.to_string();

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal login error: 2");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal login error: 3");
        }
    };

    let result = user_collection.get_user_id(requests::LoginUserRequest{ username : username.clone(), password }).await;

    let user_id = match result {
        Some(uesr_id) => uesr_id,
        None => {
            let response = responses::BadRequestResponse{ message : String::from("incorrect username or password"), code : responses::BadRequestCodes::IncorrectLoginCredentails };
            let serialized_error_response = serde_json::to_string(&response);
            let serialized_error_response = match serialized_error_response {
            Ok(response) => response,
                Err(error) => {
                    eprintln!("{}", error);
                    return HttpResponse::InternalServerError().body("Internal login error: 4");
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
            return HttpResponse::InternalServerError().body("Internal login error: 5");
        }
    };

    let response = responses::LoginResposne{ token : encoded_token };
    let serialized_response = serde_json::to_string(&response);
    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal login error: 6");
        }
    };
    
    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/get_user_page_info")]
pub async fn get_user_page_info(body: web::Json<requests::GetBasicUserRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = match lock_database(&data.db, "Internal get user page info error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal get user page info error: 2");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal get next id error: 3");
        }
    };

    let result = user_collection.get_user_page_info(user_id).await;

    let user_page_response = match result {
        Some(response) => response,
        None => {
            return HttpResponse::InternalServerError().body("Internal user page info error: 4");
        }
    };

    let serialized_response = serde_json::to_string(&user_page_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{:?}", error);
            return HttpResponse::InternalServerError().body("Internal user page info error: 5");
        }
        
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/get_next_card_id")]
pub async fn get_next_card_id(body: web::Json<requests::GetBasicUserRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = match lock_database(&data.db, "Internal get next id error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal get next id error: 2");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal get next id error: 3");
        }
    };

    let result: Option<responses::GetUserNextIdResponse> = user_collection.get_card_next_id(user_id).await;

    let next_card_id_response = match result {
        Some(response) => response,
        None => {
            return HttpResponse::InternalServerError().body("Internal get next id error: 4");
        }
    };

    let serialized_response = serde_json::to_string(&next_card_id_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{:?}", error);
            return HttpResponse::InternalServerError().body("Internal get next id error: 5");
        }
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/get_card")]
pub async fn get_card(body: web::Json<requests::GetCardRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = match lock_database(&data.db, "Internal get card error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let card_id = body.card_id;

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal get card error: 2");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal get card error: 3");
        }
    };

    let result = user_collection.get_card(user_id, card_id).await;

    let get_card_response = match result {
        Some(response) => response,
        None => {
            return HttpResponse::InternalServerError().body("Internal get card error: 4");
        }
    };

    let serialized_response = serde_json::to_string(&get_card_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{:?}", error);
            return HttpResponse::InternalServerError().body("Internal get card error: 6");
        }
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/create_card")]
pub async fn create_card(body: web::Json<requests::CreateCardRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = match lock_database(&data.db, "Internal create card error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal create card error: 2");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal create card error: 3");
        }
    };

    let result = user_collection.create_card(user_id).await;

    let create_card_response = match result {
        Some(response) => response,
        None => {
           return HttpResponse::InternalServerError().body("Internal create card error: 4");
        }     
    };

    let serialized_response = serde_json::to_string(&create_card_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{:?}", error);
            return HttpResponse::InternalServerError().body("Internal create card error: 5");
        }
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/update_card")]
pub async fn update_card(body: web::Json<requests::UpdateCardRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = match lock_database(&data.db, "Internal update card error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let card_data = &body.card_data;

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal update card error: 2");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal update card error: 3");
        }
    };

    let result = user_collection.update_card(user_id, card_data).await;

    match result {
        Ok(_) => {
            HttpResponse::Ok().content_type(ContentType::json()).body("Card updated successfuly")
        },
        Err(error) => {
            match error {
                DatabaseError::CodeParsingFailed => {
                    HttpResponse::InternalServerError().body("Internal update card error: 4")
                },
                DatabaseError::DatabaseCardUpdateFailed => {
                    HttpResponse::InternalServerError().body("Internal update card error: 5")
                }
                _ => {
                    HttpResponse::InternalServerError().body("Internal update card error: X")
                }
            }
        }
    }
}

#[post("/delete_card")]
pub async fn delete_card(body: web::Json<requests::DeleteCardRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = match lock_database(&data.db, "Internal delete card error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let card_id = body.card_id;

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal delete card error: 2");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal delete card error: 3");
        }
    };

    let result = user_collection.delete_card(user_id, card_id).await;

    match result {
             Ok(_) => {
            HttpResponse::Ok().content_type(ContentType::json()).body("Card deleted successfuly")
        },
        Err(error) => {
            match error {
                DatabaseError::CannotDeleteCardWithEmptyDeviceType => {
                    HttpResponse::InternalServerError().body("Internal delete card error: 4")
                },
                DatabaseError::CheckingCardDeviceTypeFailed => {
                    HttpResponse::InternalServerError().body("Internal delete card error: 5")
                },
                DatabaseError::UserIdFilterCreationFailed => {
                    HttpResponse::InternalServerError().body("Internal delete card error: 6")
                },
                DatabaseError::DatabaseCardUpdateFailed => {
                    HttpResponse::InternalServerError().body("Internal delete card error: 7")
                },
                _ => {
                    HttpResponse::InternalServerError().body("Internal update card error: X")
                }
            }
        }   
    }
}

#[post("/register_device")]
pub async fn register_device(body: web::Json<requests::RegisterDeviceRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = match lock_database(&data.db, "Internal device regisration error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let collection = match database::get_collection(&db, CollectionType::DeviceCollection).await {
        Ok(device_collection) => device_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal device registration error: 2");
        }
    };

    let device_collection= match collection {
        Collection::Device(device) => device,
        _ => {
            return HttpResponse::InternalServerError().body("Internal device registration error: 3");
        }
    };

    let device_id = match DeviceId::from_str(&body.device_id) {
        Some(id) => id,
        None => {
            return HttpResponse::InternalServerError().body("Internal device registration error: 4");
        }
    }; 

    let is_registered = device_collection.is_device_pressent(device_id).await;

    match is_registered {
        Some(state) => {
            if state{
                return HttpResponse::Ok().body("registered");
            }
        },
        None => {
            return HttpResponse::InternalServerError().body("Internal device registration error: 5");
        }
    };

    if device_collection.register_device(body.into_inner()).await == false {
        return HttpResponse::InternalServerError().body("Internal device registration error: 5");
    }

    HttpResponse::Ok().body("registered")
}

#[post("/debug")]
pub async fn debug(data: web::Data<Arc<AppState>>) -> impl Responder {
    HttpResponse::Ok().body(format!("Is empty: {:?}", data.generated_codes.lock().unwrap().is_empty()))
}

#[post("/generate_device_code")]
pub async fn generate_device_code(body: web::Json<requests::GenerateDeviceCode>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = match lock_database(&data.db, "Internal device code generation error: 1") {
        Ok(db) => db,
        Err(response) => return response
    };

    let collection = match database::get_collection(&db, CollectionType::DeviceCodeCollection).await {
        Ok(device_code_collection) => device_code_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal device code generation error: 2");
        }
    };

    let mut code = Code::new();
    let mut shuffle_counter : u32 = 0;
    let mut generated_codes = match data.generated_codes.lock() {
        Ok(codes) => codes,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal device code generation error: 3");
        }
    };

    loop {
        if shuffle_counter > MAX_SHUFFLE_COUNT
        {
            code = Code::new();
            shuffle_counter = 0;
        }

        if generated_codes.get(&code).is_none() {
            break;
        }
        
        code.shuffle();
        shuffle_counter += 1;
    }

    generated_codes.insert(code.clone());

    let device_code_collection= match collection {
        Collection::DeviceCode(collection) => collection,
        _ => {
            return HttpResponse::InternalServerError().body("Internal device code generation error: 4");
        }
    };

    let device_id = match DeviceId::from_str(&body.device_id) {
        Some(id) => id,
        None => {
            return HttpResponse::InternalServerError().body("Internal device code generation error: 5");
        }
    }; 

    let code_string = device_code_collection.assign_code_to_device(code, device_id).await;

    let code_string = match code_string {
        Some(code_string) => code_string,
        None => {
            return HttpResponse::InternalServerError().body("Internal device code generation error: 6");
        }
    };

    HttpResponse::Ok().body(format!("Code: {}", code_string))
}

//post - Generate a access code for device
//post - Add the device to a connection await list


// 1. create a socket connection with the device
// 2. Set the staus devie flag to online


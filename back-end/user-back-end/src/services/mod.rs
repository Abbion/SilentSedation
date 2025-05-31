// Rework 3.0

use std::sync::Arc;
use actix_web::{http::header::ContentType, post, web::{ self }, HttpResponse, Responder};
use bson::DateTime;
use crate::{code_generator::Code, communication::{ requests::{self, ConnectCardToDeviceRequest, PerformActionOnDeviceRequest }, responses }, constants::{self, MAX_SHUFFLE_COUNT}, database::{ DatabaseObjectId, DeviceId }, enums::device_actions::DeviceActionType, events::device_events::DeviceEvent, utils::device_types::DeviceType};
use crate::database::{ self, CollectionType, UserId, Collection };
use crate::database::error_types::DatabaseError;
use crate::auth;
use crate::state::AppState;

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
    let db = &data.db.lock().await;
    let username = body.username.to_string();
    let password = body.password.to_string();

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal login error: 1");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal login error: 2");
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
                    return HttpResponse::InternalServerError().body("Internal login error: 3");
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
            return HttpResponse::InternalServerError().body("Internal login error: 4");
        }
    };

    let response = responses::LoginResposne{ token : encoded_token };
    let serialized_response = serde_json::to_string(&response);
    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal login error: 5");
        }
    };
    
    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/get_user_page_info")]
pub async fn get_user_page_info(body: web::Json<requests::GetBasicUserRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = &data.db.lock().await;
    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal get user page info error: 1");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal get next id error: 2");
        }
    };

    let result = user_collection.get_user_page_info(user_id).await;

    let user_page_response = match result {
        Some(response) => response,
        None => {
            return HttpResponse::InternalServerError().body("Internal user page info error: 3");
        }
    };

    let serialized_response = serde_json::to_string(&user_page_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{:?}", error);
            return HttpResponse::InternalServerError().body("Internal user page info error: 4");
        }
        
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/get_next_card_id")]
pub async fn get_next_card_id(body: web::Json<requests::GetBasicUserRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = &data.db.lock().await;
    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal get next id error: 1");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal get next id error: 2");
        }
    };

    let result: Option<responses::GetUserNextIdResponse> = user_collection.get_card_next_id(user_id).await;

    let next_card_id_response = match result {
        Some(response) => response,
        None => {
            return HttpResponse::InternalServerError().body("Internal get next id error: 3");
        }
    };

    let serialized_response = serde_json::to_string(&next_card_id_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{:?}", error);
            return HttpResponse::InternalServerError().body("Internal get next id error: 4");
        }
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/get_card")]
pub async fn get_card(body: web::Json<requests::GetCardRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = &data.db.lock().await;
    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let card_id = body.card_id;

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal get card error: 1");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal get card error: 2");
        }
    };

    let result = user_collection.get_card(&user_id, card_id).await;

    let get_card_response = match result {
        Some(response) => response,
        None => {
            return HttpResponse::InternalServerError().body("Internal get card error: 3");
        }
    };

    let serialized_response = serde_json::to_string(&get_card_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{:?}", error);
            return HttpResponse::InternalServerError().body("Internal get card error: 4");
        }
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/create_card")]
pub async fn create_card(body: web::Json<requests::CreateCardRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = &data.db.lock().await;
    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal create card error: 1");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal create card error: 2");
        }
    };

    let result = user_collection.create_card(user_id).await;

    let create_card_response = match result {
        Some(response) => response,
        None => {
           return HttpResponse::InternalServerError().body("Internal create card error: 3");
        }     
    };

    let serialized_response = serde_json::to_string(&create_card_response);

    let serialized_response = match serialized_response {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{:?}", error);
            return HttpResponse::InternalServerError().body("Internal create card error: 4");
        }
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serialized_response)
}

#[post("/connect_card_to_device")]
pub async fn connect_card_to_device(body: web::Json<ConnectCardToDeviceRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = &data.db.lock().await;
    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let code = match Code::from_string(body.code.clone()) {
        Some(code) => code,
        None => {
            return HttpResponse::InternalServerError().body("Internal connect card to device error: 1");
        }
    };

    let collection = match database::get_collection(&db, CollectionType::DeviceCodeCollection).await {
        Ok(device_code_collection) => device_code_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal connect card to device error: 2");
        }
    };

    let device_code_collection = match collection {
        Collection::DeviceCode(device_code) => device_code,
        _ => {
            return HttpResponse::InternalServerError().body("Internal connect card to device error: 3");
        }
    };

    // I don't know if we should inform the user that a code belongs to a different device type. 
    // Now inform just about the inactive code
    let device_id = match device_code_collection.get_device_id_by_code(code, body.device_type).await {
        Some(device_id) => device_id,
        None => {
            return HttpResponse::Ok().json( responses::DeviceConnectionResponse {
                success : false,
                message : "Code inactive".to_string()
            } );
        }
    };

    device_code_collection.remove_device_by_device_id(&device_id).await;

    let collection = match database::get_collection(&db, CollectionType::DeviceCollection).await {
        Ok(device_collection) => device_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal connect card to device error: 4");
        }
    };

    let device_collection = match collection {
        Collection::Device(device) => device,
        _ => {
            return HttpResponse::InternalServerError().body("Internal connect card to device error: 5");
        }
    };

    if device_collection.assign_master_to_device(&device_id, &user_id, body.id).await == false {
        return HttpResponse::InternalServerError().body("Internal connect card to device error: 6");
    }


    HttpResponse::Ok().json( responses::DeviceConnectionResponse {
        success : true,
        message : "Card connected".to_string()
    } )
}

#[post("/update_card")]
pub async fn update_card(body: web::Json<requests::UpdateCardRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = &data.db.lock().await;
    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let card_data = &body.card_data;

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal update card error: 1");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal update card error: 2");
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
                    HttpResponse::InternalServerError().body("Internal update card error: 3")
                },
                DatabaseError::DatabaseCardUpdateFailed => {
                    HttpResponse::InternalServerError().body("Internal update card error: 10")
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
    let db = &data.db.lock().await;
    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let card_id = body.card_id;

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal delete card error: 1");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal delete card error: 2");
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

#[post("/perform_action_on_device")]
pub async fn perform_action_on_device(body: web::Json<PerformActionOnDeviceRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = &data.db.lock().await;
    let user_id = match get_user_id(&body.token, &data.jwt, "get user page info") {
        Ok(id) => id,
        Err(response) => return response
    };

    let collection = match database::get_collection(&db, CollectionType::UserCollection).await {
        Ok(user_collection) => user_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal perform action on device error: 1");
        }
    };

    let user_collection= match collection {
        Collection::User(user) => user,
        _ => {
            return HttpResponse::InternalServerError().body("Internal perform action on device error: 2");
        }
    };

    let card = match user_collection.get_card(&user_id, body.card_id).await {
        Some(card) => card,
        None => {
            return HttpResponse::InternalServerError().body("Internal perform action on device error: 3");
        }
    };

    if card.device_type.as_native_value() != body.device_type {
        return HttpResponse::InternalServerError().body("Internal perform action on device error: 4");
    }

    let action_type = match DeviceActionType::new(body.action_type) {
        DeviceActionType::None => {
            return HttpResponse::InternalServerError().body("Internal perform action on device error: 5");
        },
        DeviceActionType::Zap(_) => {
            let data = match card.device_type.clone() {
                DeviceType::ShockCaller(data) => data,
                DeviceType::Empty() => {
                    return HttpResponse::InternalServerError().body("Internal perform action on device error: 6");
                }
            };
            
            let power = match data {
                Some(data) => data.power,
                None => {
                    return HttpResponse::InternalServerError().body("Internal perform action on device error: 7");
                }
            };

            DeviceActionType::Zap(power)
        }
    };

    let collection = match database::get_collection(&db, CollectionType::DeviceCollection).await {
        Ok(device_collection) => device_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal perform action on device error: 8");
        }
    };

    let device_collection= match collection {
        Collection::Device(device) => device,
        _ => {
            return HttpResponse::InternalServerError().body("Internal perform action on device error: 9");
        }
    };

    let device_id = match device_collection.get_user_device_id_at_card_id(&user_id, body.card_id).await {
        Some(id) => id,
        None => {
            return HttpResponse::InternalServerError().body("Internal perform action on device error: 10");
        }
    };

    let device_event = DeviceEvent {
        time_stamp : DateTime::from_chrono(chrono::Utc::now()),
        device_type : card.device_type,
        action_type : action_type
    };

    let mut device_events = data.device_events.lock().await;
    device_events.insert(device_id, device_event);

    HttpResponse::Ok().content_type(ContentType::json()).body("action performed")
}

#[post("/register_device")]
pub async fn register_device(body: web::Json<requests::RegisterDeviceRequest>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = &data.db.lock().await;
    let collection = match database::get_collection(&db, CollectionType::DeviceCollection).await {
        Ok(device_collection) => device_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal device registration error: 1");
        }
    };

    let device_collection= match collection {
        Collection::Device(device) => device,
        _ => {
            return HttpResponse::InternalServerError().body("Internal device registration error: 2");
        }
    };

    let device_id = match DeviceId::from_str(&body.device_id) {
        Some(id) => id,
        None => {
            return HttpResponse::InternalServerError().body("Internal device registration error: 3");
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
            return HttpResponse::InternalServerError().body("Internal device registration error: 4");
        }
    };

    if device_collection.register_device(body.into_inner()).await == false {
        return HttpResponse::InternalServerError().body("Internal device registration error: 5");
    }

    HttpResponse::Ok().content_type(ContentType::plaintext()).body("registered")
}

#[post("/generate_device_code")]
pub async fn generate_device_code(body: web::Json<requests::GenerateDeviceCode>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let db = &data.db.lock().await;
    let collection = match database::get_collection(&db, CollectionType::DeviceCodeCollection).await {
        Ok(device_code_collection) => device_code_collection,
        Err(error) => {
            eprintln!("{}", error);
            return HttpResponse::InternalServerError().body("Internal device code generation error: 1");
        }
    };

    let mut code = Code::new();
    let mut shuffle_counter : u32 = 0;
    let mut generated_codes = data.generated_codes.lock().await;

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

    let device_code_collection= match collection {
        Collection::DeviceCode(collection) => collection,
        _ => {
            return HttpResponse::InternalServerError().body("Internal device code generation error: 2");
        }
    };

    let device_id = match DeviceId::from_str(&body.device_id) {
        Some(id) => id,
        None => {
            return HttpResponse::InternalServerError().body("Internal device code generation error: 3");
        }
    }; 

    let code_string = device_code_collection.assign_code_to_device(code.clone(), device_id, body.device_type).await;

    let code_string = match code_string {
        Some(code_string) => code_string,
        None => {
            return HttpResponse::InternalServerError().body("Internal device code generation error: 4");
        }
    };

    generated_codes.insert(code);
    HttpResponse::Ok().content_type(ContentType::plaintext()).body(format!("Code: {}", code_string))
}
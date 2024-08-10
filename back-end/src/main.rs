use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{get, http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder};
use communication::{ requests, responses };
use database::UserId;
use mongodb::Database;
use private::PrivateKeys;

mod auth;
mod private;
mod communication;
mod database;

struct AppState {
    jwt : auth::jwt::JsonWebTokenData,
    db : Mutex<Database>
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
async fn login_user(body: web::Json<requests::LoginUserRequest>, data: web::Data<AppState>) -> impl Responder {
    let db = match data.db.lock() {
        Ok(db) => db,
        Err(e) => {
            eprint!("{}", e);
            return HttpResponse::InternalServerError().body("Internal login error: 1");
        }
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
    let db = match data.db.lock() {
        Ok(db) => db,
        Err(e) => {
            eprint!("{}", e);
            return HttpResponse::InternalServerError().body("Internal user page info error: 1");
        }
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
            return HttpResponse::InternalServerError().body("Internal user page info error: 2");
        }
    };

    let serialized_response = serde_json::to_string(&user_page_response);

    println!("{:?}", serialized_response);

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
async fn get_new_card_id(body: web::Json<requests::GetBasicUserRequest>, data: web::Data<AppState>) -> impl Responder {
    let token = &body.token;
    
    let user_token_result = data.jwt.decode::<auth::jwt::UserToken>(token.to_owned());

    let user_token = match user_token_result {
        Ok(user_token) => user_token.claims,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().body("User token dedode failed");
        }
    };

    let next_id_respose = 1;
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
        .service(get_new_card_id)
        .route("/hey", web::get().to(manual_hello))
    })
    .workers(4)
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}
use actix_cors::Cors;
use actix_web::{get, post, web::{self}, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{ Deserialize, Serialize };

mod auth;
mod private;

struct AppState {
    jwt : auth::jwt::JsonWebTokenData
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginData {
    username : String,
    password: String,
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

        match encoded_token {
            Ok(token) => {
                return HttpResponse::Ok().body(format!("token : {}", token));
            },
            Err(e) => {
                println!("{}", e);
                HttpResponse::NotFound().body("Internal login error");
            }
        }
    }

    HttpResponse::NotFound().body("Bad login credentails")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let private_keys = private::get_private_keys();

    if (private_keys.is_err()) {
        let error = private_keys.err().unwrap();
        println!("{}", error);
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "private_keys were not created!",
        ));
    }

    let private_keys: private::PrivateKeys = private_keys.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allow_any_header()
                    .max_age(3600)
            )
        .app_data(web::Data::new(AppState{
            jwt : auth::jwt::JsonWebTokenData::new(&private_keys)
        }))
        .service(login_user)
        .service(hello)
        .service(path_extractor)
        .route("/hey", web::get().to(manual_hello))
    })
    .workers(4)
    .bind(("127.0.0.1", 90))?
    .run()
    .await
}
use actix_cors::Cors;
use actix_web::{get, http, post, web, web::Json, HttpRequest, App, HttpResponse, HttpServer, Responder};
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
struct LoginData {
    username : String,
    password: String
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
async fn login_user(req: HttpRequest, body: web::Json<LoginData>) -> impl Responder {
    
    println!("{:?} -> {:?}",  req, body);
    let username = &body.username;
    let password = &body.password;
    
    if username == "Wiktor" && password == "123" { 
        return HttpResponse::Ok().body("Token");
    }

    HttpResponse::NotFound().body("Bad login credentails")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allow_any_header()
                    .max_age(3600)
            )
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
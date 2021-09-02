use std::collections::HashMap;
use actix_web::{
    dev::BodyEncoding, get, http::ContentEncoding, middleware, App, HttpResponse, HttpServer,
};
use mmm_core::{turn_on, turn_off};

pub const INDEX: &str = include_str!("index.html");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index)
            .service(on)
            .service(off)
    })
    .bind("127.0.0.1:8080")?
    .disable_signals()
    .run()
    .await
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Gzip)
        .body(INDEX)
}

#[get("/on")]
async fn on() -> HttpResponse {
    let mut results: HashMap<String, String> = HashMap::new();

    match turn_on() {
        Ok(_) => {
            results.insert("status".to_string(), "0".to_string());
            results.insert("message".to_string(), "working!".to_string());

            let json = serde_json::to_string(&results).unwrap();

            let response = HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body(json);

            return response;
        }
        Err((error, code)) => {
            results.insert("status".to_string(), code.to_string());
            results.insert("message".to_string(), error.to_string());

            let json = serde_json::to_string(&results).unwrap();

            let response = HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body(json);

            return response;
        }
    };
}

#[get("/off")]
async fn off() -> HttpResponse {
    let mut results: HashMap<String, String> = HashMap::new();

    match turn_off() {
        Ok(_) => {
            results.insert("status".to_string(), "0".to_string());
            results.insert("message".to_string(), "working!".to_string());

            let json = serde_json::to_string(&results).unwrap();

            let response = HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body(json);

            return response;
        }
        Err((error, code)) => {
            results.insert("status".to_string(), code.to_string());
            results.insert("message".to_string(), error.to_string());

            let json = serde_json::to_string(&results).unwrap();

            let response = HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body(json);

            return response;
        }
    };
}
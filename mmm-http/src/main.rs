#[macro_use]
extern crate lazy_static;

use actix_web::{
    dev::BodyEncoding, get, http::ContentEncoding, middleware, App, HttpResponse, HttpServer,
};
use mmm_core::Switcher;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use std::collections::HashMap;

lazy_static! {
    static ref INDEX: String = format!("<html>\n{}\n</html>", include_str!("index.html"));
}

pub static SWITCHER: Lazy<RwLock<Switcher>> =
    Lazy::new(|| RwLock::new(Switcher::new()));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index)
            .service(on)
            .service(off)
            .service(data)
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
        .body(INDEX.to_string())
}

#[get("/on")]
async fn on() -> HttpResponse {
    match SWITCHER.write().unwrap().on() {
        Ok(_) => HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body("{\"status\":\"0\",\"message\":\"working!\"}"),
        Err((error, code)) => HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body(json_error(error, code)),
    }
}

#[get("/off")]
async fn off() -> HttpResponse {
    match SWITCHER.write().unwrap().off() {
        Ok(_) => HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body("{\"status\":\"0\",\"message\":\"working!\"}"),
        Err((error, code)) => HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body(format!("{{\"status\":\"{}\",\"message\":\"{}\"}}", code, error)),
    }
}

#[get("/data.csv")]
async fn data() -> HttpResponse {
    let mut output = String::new();

    for (date, state) in mmm_core::get_data() {
        output.push_str(&format!("{},{}\n", date, state))
    }

    HttpResponse::Ok()
        .encoding(ContentEncoding::Gzip)
        .body(output)
}

fn json_error(error: String, code: usize) -> String {
    let mut results: HashMap<String, String> = HashMap::new();

    results.insert("status".to_string(), code.to_string());
    results.insert("message".to_string(), error.to_string());

    let json = serde_json::to_string(&results).unwrap();

    return json;
}
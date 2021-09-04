#[macro_use]
extern crate lazy_static;

use actix_web::{
    dev::BodyEncoding, get, http::ContentEncoding, middleware, App, HttpResponse, HttpServer,
};
use mmm_core::Switcher;
use std::sync::RwLock;
use once_cell::sync::Lazy;

lazy_static! {
    static ref INDEX: String = format!("<html>\n<head>\n<style type=\"text/css\">\n{}</style>\n</head>\n<body>\n{}\n<script type=\"module\">\n{}\n</script>\n</body>\n</html>", include_str!("style.css"), include_str!("index.html"), include_str!("index.js"));
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
            .service(sprinkler_state)
            .service(clear)
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
            .body(format!("{{\"status\":\"{}\",\"message\":\"{}\"}}", code, error)),
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
        let state = match state {
            true => "on",
            false => "off",
        };

        output.push_str(&format!("{},{}\n", date, state));
    }

    HttpResponse::Ok()
        .encoding(ContentEncoding::Gzip)
        .body(output)
}

#[get("/state")]
async fn sprinkler_state() -> HttpResponse {
    let state = match SWITCHER.read().unwrap().state {
        true => "on",
        false => "off",
    };

    HttpResponse::Ok()
        .encoding(ContentEncoding::Gzip)
        .body(state)
}

#[get("/clear")]
async fn clear() -> HttpResponse {
    match SWITCHER.write().unwrap().config.write_clear() {
        Ok(_) => HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body("{\"status\":\"0\",\"message\":\"working!\"}"),
        Err(error) => HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body(format!("{{\"status\":\"1\",\"message\":\"{}\"}}", error.to_string())),
    }
}
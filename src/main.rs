#[macro_use]
extern crate lazy_static;

use actix_web::{App, HttpResponse, HttpServer, body::Body, dev::BodyEncoding, get, http::ContentEncoding, web::Bytes};
use chrono::prelude::*;

mod tools;

static mut STATE: bool = false;

lazy_static! {
    static ref INDEX: String = format!("<html>\n<head>\n<title>Sprinkler Control</title>\n<link rel=\"icon\" href=\"favicon.ico\"/>\n<style type=\"text/css\">\n{}\n</style>\n</head>\n<body>\n{}\n<script type=\"module\">\n{}\n</script>\n</body>\n</html>", include_str!("style.css"), include_str!("index.html"), include_str!("index.js"));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut dir = dirs::config_dir().unwrap();
    dir.push("mmm.csv");

    unsafe {
        STATE = match fstream::read_text(dir).unwrap_or("Off".to_string()).lines().last().unwrap_or("Off").split(',').last().unwrap_or("Off") {
            "On" => true,
            "Off" => false,
            &_ => false,
        };
    }

    HttpServer::new(||
        App::new()
            .service(index)
            .service(favicon)
            .service(weather)
            .service(off)
            .service(on)
            .service(state)
            .service(data)
            .service(toggle)
            .service(clear)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Gzip)
        .body(INDEX.to_string())
}

#[get("/favicon.ico")]
async fn favicon() -> HttpResponse {
    HttpResponse::Ok()
    .encoding(ContentEncoding::Gzip)
    .body(Body::Bytes(Bytes::from(include_bytes!("favicon.ico").to_vec())))
}

#[get("/weather")]
async fn weather() -> HttpResponse {
    let weather = tools::Weather::now().await;

    HttpResponse::Ok()
        .encoding(ContentEncoding::Gzip)
        .body(format!("Temperature: {}, {}.", weather.temperature, weather.description))
}

#[get("/on")]
async fn on() -> HttpResponse {
    unsafe {
        match STATE {
            true => (),
            false => {
                println!("turning on");
                STATE = true;
                update_database().await;
            }
        }

        HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body(format!("{{\"state\":\"On\"}}"))
    }
}

#[get("/off")]
async fn off() -> HttpResponse {
    unsafe {
        match STATE {
            true => {
                println!("turning off");
                STATE = false;
                update_database().await;
            }
            false => (),
        }

        HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body(format!("{{\"state\":\"Off\"}}"))
    }
}

#[get("/toggle")]
async fn toggle() -> HttpResponse {
    unsafe {
        match STATE {
            true =>  {
                println!("turning off");
                STATE = false;
                update_database().await;
            },
            false =>  {
                println!("turning on");
                STATE = true;
                update_database().await; 
            },
        }

        let current_state = match STATE {
            true => "On",
            false => "Off",
        };

        HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body(format!("{{\"state\":\"{}\"}}", current_state))
    }
}

#[get("/state")]
async fn state() -> HttpResponse {
    unsafe {
        let state = match STATE {
            true => "On",
            false => "Off",
        };

        HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body(format!("{{\"state\":\"{}\"}}", state)) 
    }
}

#[get("/clear")]
async fn clear() -> HttpResponse {
    let mut dir = dirs::config_dir().unwrap();
    dir.push("mmm.csv");

    std::fs::remove_file(dir).unwrap();
    
    HttpResponse::Ok()
        .encoding(ContentEncoding::Gzip)
        .body(format!("{{\"status\":\"0\"}}"))
}

#[get("/data.csv")]
async fn data() -> HttpResponse {
    let mut dir = dirs::config_dir().unwrap();
    dir.push("mmm.csv");

    if dir.exists() {
        let data = fstream::read_text(dir).unwrap();

        return HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body(data);
    } else {
        fstream::write_text(dir, "time,status,temperature", false);

        return HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body("time,status,temperature"); 
    }
}

async fn update_database() {
    unsafe {
        let current_state = match STATE {
            true => "On",
            false => "Off",
        };

        let now = Utc::now();

        let mut dir = dirs::config_dir().unwrap();
        dir.push("mmm.csv");

        let current_weather = tools::Weather::now().await;

        let output = format!("\n{},{},{}", now, current_state, current_weather.temperature);

        fstream::write_text(dir, output, false).unwrap();
    }
}

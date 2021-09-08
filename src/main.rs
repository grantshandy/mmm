#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use std::str::FromStr;

use actix_web::{App, HttpResponse, HttpServer, body::Body, dev::BodyEncoding, get, http::ContentEncoding, web::Bytes};
use chrono::prelude::*;

mod tools;

static mut STATE: bool = false;

lazy_static! {
    static ref INDEX: String = format!("<html>\n<head>\n<title>Sprinkler Control</title>\n<link rel=\"icon\" href=\"favicon.ico\"/>\n<style type=\"text/css\">\n{}\n</style>\n</head>\n<body>\n{}\n<script type=\"module\">\n{}\n</script>\n</body>\n</html>", include_str!("style.css"), include_str!("index.html"), include_str!("index.js"));
    static ref CSV_PATH: PathBuf = {
        let mut dir = dirs::config_dir().unwrap();
        dir.push("mmm.csv");
        dir
    };
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
            .service(gallons)
            .service(vulf_sans_regular)
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

#[get("/Vulf_Sans-Regular.woff2")]
async fn vulf_sans_regular() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Gzip)
        .body(Body::Bytes(Bytes::from(include_bytes!("fonts/Vulf_Sans-Regular.woff2").to_vec())))
}

#[get("/weather")]
async fn weather() -> HttpResponse {
    let weather = tools::Weather::now().await;

    HttpResponse::Ok()
        .encoding(ContentEncoding::Gzip)
        .body(format!("Temperature: {}, {} °C.", weather.temperature, weather.description))
}

#[get("/gallons")]
async fn gallons() -> HttpResponse {
    let mut gallon_vec: Vec<(DateTime<Utc>, bool)> = Vec::new();

    if !CSV_PATH.exists() {
        return HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body("{{\"gallons\":\"0\"}}");
    }

    let csv_str = fstream::read_text(CSV_PATH.clone()).unwrap();

    for x in csv_str.lines().skip(1) {
        let time = x.split(',').nth(0).unwrap();

        let this_state: bool = match x.split(',').nth(2).unwrap() {
            "On" => true,
            "Off" => false,
            &_ => false,
        };

        let time = match DateTime::from_str(time) {
            Ok(other_data) => other_data,
            Err(error) => panic!("error: {}, time: {}", error, time),
        };

        gallon_vec.push((time, this_state));
    }

    let mut total_gallons: f64 = 0.0;

    let current_state: bool;

    unsafe {
        current_state = STATE;
    }

    for (time, this_state) in gallon_vec {
        unsafe {
            if this_state = STATE {
                
            }
        }
    }

    HttpResponse::Ok()
        .encoding(ContentEncoding::Gzip)
        .body(format!("Temperature"))
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
    std::fs::remove_file(CSV_PATH.clone()).unwrap();
    
    HttpResponse::Ok()
        .encoding(ContentEncoding::Gzip)
        .body(format!("{{\"status\":\"0\"}}"))
}

#[get("/data.csv")]
async fn data() -> HttpResponse {
    if CSV_PATH.exists() {
        let data = fstream::read_text(CSV_PATH.clone()).unwrap();

        return HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body(data);
    } else {
        fstream::write_text(CSV_PATH.clone(), "time,temperature,status", false);

        return HttpResponse::Ok()
            .encoding(ContentEncoding::Gzip)
            .body("time,temperature,status"); 
    }
}

async fn update_database() {
    unsafe {
        let current_state = match STATE {
            true => "On",
            false => "Off",
        };

        let now = Utc::now();
        let current_weather = tools::Weather::now().await;
        let output = format!("\n{},{} °C,{}", now, current_weather.temperature, current_state);

        fstream::write_text(CSV_PATH.clone(), output, false).unwrap();
    }
}

use std::path::PathBuf;

mod electronics;
mod weather;
mod web;

use chrono::prelude::*;
use tiny_http::Server;
use weather::Weather;

pub static mut STATE: bool = false;

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("started server");

    let mut path = dirs::home_dir().unwrap();
    path.push("mmm.csv");

    unsafe {
        STATE = match fstream::read_text(&path)
            .unwrap_or("Off".to_string())
            .lines()
            .last()
            .unwrap_or("Off")
            .split(',')
            .last()
            .unwrap_or("Off")
        {
            "On" => true,
            "Off" => false,
            &_ => false,
        };
    }

    for request in server.incoming_requests() {
        match request.url() {
            "/toggle" => request.respond(web::toggle(&path)).unwrap(),
            "/state" => request.respond(web::state()).unwrap(),
            "/clear" => request.respond(web::clear(&path)).unwrap(),
            "/weather" => request.respond(web::weather()).unwrap(),
            "/data.csv" => request.respond(web::data(&path)).unwrap(),
            "/favicon.ico" => request.respond(web::favicon()).unwrap(),
            "/Vulf_Sans-Regular.woff2" => request.respond(web::font()).unwrap(),
            &_ => request.respond(web::index()).unwrap(),
        }
    }
}

fn update_database(path: &PathBuf) {
    unsafe {
        let current_state = match STATE {
            true => "On",
            false => "Off",
        };

        let now = Utc::now();
        let current_weather = Weather::now();
        let output = format!(
            "\n{},{} °C,{}",
            now, current_weather.temperature, current_state
        );

        fstream::write_text(path, output, false).unwrap();
    }
}

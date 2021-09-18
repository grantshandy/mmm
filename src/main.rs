#[macro_use]
extern crate clap;

use std::path::PathBuf;

mod electronics;
mod weather;
mod web;

use chrono::prelude::*;
use clap::Arg;
use tiny_http::Server;
use weather::Weather;

pub static mut STATE: bool = false;
pub static mut PIN: u8 = 17;

fn main() {
    let app = app_from_crate!()
        .arg(
            Arg::with_name("pin")
                .help("port that turns the solenoid on and off.")
                .takes_value(true)
                .long("pin"),
        )
        .arg(
            Arg::with_name("port")
                .help("port that the webserver is served on.")
                .takes_value(true)
                .long("port"),
        )
        .get_matches();

    match app.value_of("pin") {
        Some(pin) => unsafe { PIN = pin.parse::<u8>().expect("pin must be a number.") },
        None => (),
    };

    let port = match app.value_of("port") {
        Some(data) => data.parse::<usize>().expect("port must be a number."),
        None => 8080,
    };

    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();
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
            "/graph.svg" => request.respond(web::graph(&path)).unwrap(),
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

        let now = Utc::now().to_rfc3339();
        let current_weather = Weather::now();
        let output = format!(
            "\n{},{} °C,{}",
            now, current_weather.temperature, current_state
        );

        fstream::write_text(path, output, false).unwrap();
    }
}
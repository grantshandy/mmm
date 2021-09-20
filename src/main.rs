#[macro_use]
extern crate clap;

use std::path::PathBuf;

mod electronics;
mod weather;
mod web;
mod graph;

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
    println!("started server on port {}", port);

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
            &_ => {
                // my god this code is terrible I should have learned regex instead.
                let split = request.url().split("_");
                if split.clone().count() == 3{
                    let length: usize = split.clone().nth(1).unwrap().parse().unwrap();
                    let res = split.last().unwrap().replace(".svg", "");
                    let mut res_split = res.split("x");

                    let width = res_split.nth(0).unwrap();
                    let width = width.parse::<usize>().unwrap();

                    let height = res_split.last().unwrap();
                    let height = height.parse::<usize>().unwrap();
                    request.respond(web::get_graph_response(&path, length, width, height)).unwrap();

                } else {
                    request.respond(web::index()).unwrap();
                }
            },
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
        // let current_weather = Weather::now();
        let output = format!(
            "\n{},weather_would_go_here_but_its_not_working,{}",
            now,
            current_state
        );

        fstream::write_text(path, output, false).unwrap();
    }
}

use std::{fs, io::Cursor, path::PathBuf};

use chrono::prelude::*;
use rppal::gpio::Gpio;
use serde_json::Value;
use tiny_http::{Response, Server};

static mut STATE: bool = false;
const PIN: u8 = 11;

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("started server");

    let mut path = PathBuf::new();
    path.push("/home/grant/");
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
            "/toggle" => request.respond(toggle(&path)).unwrap(),
            "/state" => request.respond(state()).unwrap(),
            "/clear" => request.respond(clear(&path)).unwrap(),
            "/weather" => request.respond(weather()).unwrap(),
            "/data.csv" => request.respond(data(&path)).unwrap(),
            "/favicon.ico" => request.respond(favicon()).unwrap(),
            "/Vulf_Sans-Regular.woff2" => request.respond(font()).unwrap(),
            &_ => request.respond(index()).unwrap(),
        }
    }
}

fn index() -> Response<Cursor<Vec<u8>>> {
    Response::from_string(&format!("<html>\n<head>\n<title>Sprinkler Control</title>\n<link rel=\"icon\" href=\"favicon.ico\"/>\n<style type=\"text/css\">\n{}\n</style>\n</head>\n<body>\n{}\n<script type=\"module\">\n{}\n</script>\n</body>\n</html>", include_str!("style.css"), include_str!("index.html"), include_str!("index.js")))
    .with_header(
        "Content-type: text/html"
            .parse::<tiny_http::Header>()
            .unwrap(),
    )
}

fn weather() -> Response<Cursor<Vec<u8>>> {
    let weather = Weather::now();

    Response::from_string(&format!(
        "Temperature: {} ℃, {}.",
        weather.temperature, weather.description
    ))
    .with_header(
        "Content-type: text/html"
            .parse::<tiny_http::Header>()
            .unwrap(),
    )
}

fn favicon() -> Response<Cursor<Vec<u8>>> {
    Response::from_data(include_bytes!("favicon.ico").to_vec()).with_header(
        "Content-type: image/ico"
            .parse::<tiny_http::Header>()
            .unwrap(),
    )
}

fn font() -> Response<Cursor<Vec<u8>>> {
    Response::from_data(include_bytes!("Vulf_Sans-Regular.woff2").to_vec()).with_header(
        "Content-type: image/ico"
            .parse::<tiny_http::Header>()
            .unwrap(),
    )
}

fn data(path: &PathBuf) -> Response<Cursor<Vec<u8>>> {
    if path.exists() {
        let data = fstream::read_text(path).unwrap();

        return Response::from_string(data).with_header(
            "Content-type: text/csv"
                .parse::<tiny_http::Header>()
                .unwrap(),
        );
    } else {
        fstream::write_text(path, "time,temperature,status", false);

        return Response::from_string("time,temperature,status").with_header(
            "Content-type: text/csv"
                .parse::<tiny_http::Header>()
                .unwrap(),
        );
    }
}

fn state() -> Response<Cursor<Vec<u8>>> {
    unsafe {
        let state = match STATE {
            true => "On",
            false => "Off",
        };

        return Response::from_string(format!("{{\"state\":\"{}\"}}", state)).with_header(
            "Content-type: text/json"
                .parse::<tiny_http::Header>()
                .unwrap(),
        );
    }
}

fn clear(path: &PathBuf) -> Response<Cursor<Vec<u8>>> {
    fs::remove_file(path).unwrap();

    return Response::from_string(format!("{{\"status\":\"0\"}}")).with_header(
        "Content-type: text/json"
            .parse::<tiny_http::Header>()
            .unwrap(),
    );
}

fn toggle(path: &PathBuf) -> Response<Cursor<Vec<u8>>> {
    unsafe {
        match STATE {
            true => turn_pins_off(path),
            false => turn_pins_on(path),
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
            "\n{},{} ℃,{}",
            now, current_weather.temperature, current_state
        );

        fstream::write_text(path, output, false).unwrap();
    }
}

unsafe fn turn_pins_off(path: &PathBuf) -> Response<Cursor<Vec<u8>>> {
    match Gpio::new() {
        Ok(gpio) => {
            let mut pin = match gpio.get(PIN) {
                Ok(x) => x,
                Err(error) => {
                    let current_state = match STATE {
                        true => "On",
                        false => "Off",
                    };

                    eprintln!("couldn't get the pin!! msg: {}", error);
                    return Response::from_string(&format!(
                        "{{\"state\":\"{}\",\"error\":\"{}\"}}",
                        current_state, error
                    ))
                    .with_header(
                        "Content-type: text/json"
                            .parse::<tiny_http::Header>()
                            .unwrap(),
                    );
                }
            }
            .into_output();

            pin.set_low();
            println!("turned off");
            STATE = false;
            update_database(path);

            let current_state = match STATE {
                true => "On",
                false => "Off",
            };

            return Response::from_string(&format!("{{\"state\":\"{}\"}}", current_state))
                .with_header(
                    "Content-type: text/json"
                        .parse::<tiny_http::Header>()
                        .unwrap(),
                );
        }
        Err(error) => {
            let current_state = match STATE {
                true => "On",
                false => "Off",
            };

            return Response::from_string(&format!(
                "{{\"state\":\"{}\",\"error\":\"{}\"}}",
                current_state, error
            ))
            .with_header(
                "Content-type: text/json"
                    .parse::<tiny_http::Header>()
                    .unwrap(),
            );
        }
    };
}

unsafe fn turn_pins_on(path: &PathBuf) -> Response<Cursor<Vec<u8>>> {
    match Gpio::new() {
        Ok(gpio) => {
            let mut pin = match gpio.get(PIN) {
                Ok(x) => x,
                Err(error) => {
                    let current_state = match STATE {
                        true => "On",
                        false => "Off",
                    };

                    eprintln!("couldn't get the pin!! msg: {}", error);
                    return Response::from_string(&format!(
                        "{{\"state\":\"{}\",\"error\":\"{}\"}}",
                        current_state, error
                    ))
                    .with_header(
                        "Content-type: text/json"
                            .parse::<tiny_http::Header>()
                            .unwrap(),
                    );
                }
            }
            .into_output();

            pin.set_high();
            println!("turned on");
            STATE = true;
            update_database(path);

            let current_state = match STATE {
                true => "On",
                false => "Off",
            };

            return Response::from_string(&format!("{{\"state\":\"{}\"}}", current_state))
                .with_header(
                    "Content-type: text/json"
                        .parse::<tiny_http::Header>()
                        .unwrap(),
                );
        }
        Err(error) => {
            let current_state = match STATE {
                true => "On",
                false => "Off",
            };

            return Response::from_string(&format!(
                "{{\"state\":\"{}\",\"error\":\"{}\"}}",
                current_state, error
            ))
            .with_header(
                "Content-type: text/json"
                    .parse::<tiny_http::Header>()
                    .unwrap(),
            );
        }
    };
}

pub struct Weather {
    pub temperature: f64,
    pub description: String,
}

impl Weather {
    pub fn now() -> Self {
        let resp = ureq::get(&format!(
            "http://api.weatherapi.com/v1/current.json?key={}&q=millcreek&aqi=no",
            include_str!("weather_key")
        ))
        .call()
        .unwrap()
        .into_string()
        .unwrap();

        let resp: Value = serde_json::from_str(&resp).unwrap();

        let description = match &resp["current"]["condition"]["text"] {
            Value::String(desc) => desc.clone(),
            _ => panic!("no text!"),
        };

        let temperature = match &resp["current"]["temp_c"] {
            Value::Number(desc) => match desc.as_f64() {
                Some(data) => data,
                None => panic!("not f64!!"),
            },
            _ => panic!("no temp!"),
        };

        return Self {
            temperature,
            description,
        };
    }
}

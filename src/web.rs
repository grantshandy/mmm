use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

use tiny_http::{Header, Response};
use chrono::prelude::*;

use crate::electronics;
use crate::weather::Weather;
use crate::STATE;

pub fn index() -> Response<Cursor<Vec<u8>>> {
    Response::from_string(&format!("<html>\n<head>\n<title>Sprinkler Control</title>\n<link rel=\"icon\" href=\"favicon.ico\"/>\n<style type=\"text/css\">\n{}\n</style>\n</head>\n<body>\n{}\n<script type=\"module\">\n{}\n</script>\n</body>\n</html>", include_str!("web/style.css"), include_str!("web/index.html"), include_str!("web/index.js")))
    .with_header(
        "Content-type: text/html; charset=\"UTF-8\""
            .parse::<tiny_http::Header>()
            .unwrap(),
    )
}

pub fn weather() -> Response<Cursor<Vec<u8>>> {
    let weather = Weather::now();

    Response::from_string(&format!(
        "Temperature: {} °C, {}.",
        weather.temperature, weather.description
    ))
}

pub fn favicon() -> Response<Cursor<Vec<u8>>> {
    Response::from_data(include_bytes!("web/favicon.ico").to_vec()).with_header(
        "Content-type: image/ico"
            .parse::<tiny_http::Header>()
            .unwrap(),
    )
}

pub fn font() -> Response<Cursor<Vec<u8>>> {
    Response::from_data(include_bytes!("web/Vulf_Sans-Regular.woff2").to_vec()).with_header(
        "Content-type: image/ico"
            .parse::<tiny_http::Header>()
            .unwrap(),
    )
}

pub fn data(path: &PathBuf) -> Response<Cursor<Vec<u8>>> {
    if path.exists() {
        let data = fstream::read_text(path).unwrap();

        return Response::from_string(data).with_header(
            "Content-type: text/csv; charset=utf8"
                .parse::<tiny_http::Header>()
                .unwrap(),
        );
    } else {
        fstream::write_text(path, "time,temperature,status", false);

        return Response::from_string("time,temperature,status").with_header(
            "Content-type: text/csv; charset=utf8"
                .parse::<tiny_http::Header>()
                .unwrap(),
        );
    }
}

pub fn state() -> Response<Cursor<Vec<u8>>> {
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

pub fn clear(path: &PathBuf) -> Response<Cursor<Vec<u8>>> {
    // let current_time = Utc::now();

    // let mut archives_dir = dirs::home_dir().unwrap();
    // archives_dir.push("mmm-archives");

    // if !archives_dir.exists() {
    //     fs::create_dir(&archives_dir).unwrap();
    // }

    // let mut archive_path = archives_dir;
    // archive_path.push(&format!("mmm-{}.csv", current_time));

    // fstream::write_text(path, "time,temperature,status", false);
    // fstream::write_text(archive_path, csv_str, false).expect("couldn't write the text");

    fs::remove_file(path).unwrap();

    return Response::from_string(format!("{{\"status\":\"0\"}}")).with_header(
        "Content-type: text/json"
            .parse::<tiny_http::Header>()
            .unwrap(),
    );
}

pub fn toggle(path: &PathBuf) -> Response<Cursor<Vec<u8>>> {
    unsafe {
        match STATE {
            true => {
                #[cfg(target_arch = "arm")]
                let res = electronics::turn_pins_off(path);
                #[cfg(target_arch = "x86_64")]
                let res = electronics::fake_turn_pins_off(path);

                match res {
                    Ok(current_state) => {
                        let current_state = match current_state {
                            true => "On",
                            false => "Off",
                        };

                        return Response::from_string(&format!(
                            "{{\"state\":\"{}\"}}",
                            current_state
                        ))
                        .with_header("Content-type: text/json".parse::<Header>().unwrap());
                    }
                    Err((current_state, error)) => {
                        let current_state = match current_state {
                            true => "On",
                            false => "Off",
                        };

                        return Response::from_string(&format!(
                            "{{\"state\":\"{}\",\"error\":\"{}\"}}",
                            current_state, error
                        ))
                        .with_header("Content-type: text/json".parse::<Header>().unwrap());
                    }
                };
            }
            false => {
                #[cfg(target_arch = "arm")]
                let res = electronics::turn_pins_on(path);
                #[cfg(target_arch = "x86_64")]
                let res = electronics::fake_turn_pins_on(path);

                match res {
                    Ok(current_state) => {
                        let current_state = match current_state {
                            true => "On",
                            false => "Off",
                        };

                        return Response::from_string(&format!(
                            "{{\"state\":\"{}\"}}",
                            current_state
                        ))
                        .with_header("Content-type: text/json".parse::<Header>().unwrap());
                    }
                    Err((current_state, error)) => {
                        let current_state = match current_state {
                            true => "On",
                            false => "Off",
                        };

                        return Response::from_string(&format!(
                            "{{\"state\":\"{}\",\"error\":\"{}\"}}",
                            current_state, error
                        ))
                        .with_header("Content-type: text/json".parse::<Header>().unwrap());
                    }
                };
            }
        }
    }
}

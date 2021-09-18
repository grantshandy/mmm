use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

use chrono::prelude::*;
use charts::{Chart, ScaleLinear, AreaSeriesView};
use tiny_http::{Header, Response};

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


pub fn graph(path: &PathBuf) -> Response<Cursor<Vec<u8>>> {
    let plaintext = fstream::read_text(path).unwrap();
    let mut main_data: Vec<(f32, bool)> = Vec::new();

    let width = 500;
    let height = 500;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    let min_time = -10.0;
    let max_time = 0.0;
    let off_num: f32 = 0.25;
    let on_num: f32 = 0.75;

    println!("\n");
    for line in plaintext.lines().skip(1) {
        let mut line = line.split(",");

        let date = DateTime::parse_from_rfc3339(line.nth(0).unwrap()).unwrap().time();
        let state = match line.last().unwrap() {
            "On" => true,
            "Off" => false,
            &_ => false,
        };
        let now = Utc::now().time();

        if date.signed_duration_since(now).num_minutes() > min_time as i64 {
            let date = date.signed_duration_since(now).num_seconds() as f32 / 60.0;
            main_data.push((date, state));
            println!("adding {}, {}", date, state);
        } else {
            main_data.push((min_time, state))
        }
    }
    println!("\n");

    let mut line_data: Vec<(f32, f32)> = Vec::new();

    if main_data.len() != 0 {
        if main_data.last().unwrap().0 != max_time {
            match main_data.first().unwrap().1 {
                true => line_data.push((min_time, on_num)),
                false => line_data.push((min_time, off_num)),
            }
        }
    } else {
        unsafe {
            match STATE {
                true => {
                    line_data.push((min_time, on_num));
                    line_data.push((max_time, on_num));
                }
                false => {
                    line_data.push((min_time, off_num));
                    line_data.push((max_time, off_num));
                }
            }
        }
    }

    'main_loop: for (num, (date, state)) in main_data.iter().enumerate() {
        let date = *date;
        let state = *state;

        if num != main_data.len() - 1 && num != 0 && main_data.get(num - 1).unwrap().1 == state {
            continue 'main_loop;
        }

        let coord: (f32, f32) = match state {
            true => (date, on_num),
            false => (date, off_num),
        };
        
        if num != 0 && main_data.get(num - 1).unwrap().1 != state {
            let other_coord: (f32, f32) = match state {
                true => (date, off_num),
                false => (date, on_num),
            };
    
            line_data.push(other_coord);
        }

        line_data.push(coord);

    }

    if main_data.len() != 0 {
        if main_data.last().unwrap().0 != max_time {
            match main_data.last().unwrap().1 {
                true => line_data.push((max_time, on_num)),
                false => line_data.push((max_time, off_num)),
            }
        }
    }

    let x = ScaleLinear::new()
        .set_domain(vec![min_time, max_time])
        .set_range(vec![0, width - left - right]);

    let y = ScaleLinear::new()
        .set_domain(vec![0.0, 1.0])
        .set_range(vec![height - top - bottom, 0]);

    let line_view = AreaSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        // .set_marker_type(MarkerType::None)
        .set_label_visibility(false)
        // .set_colors(colors)
        .load_data(&line_data).unwrap();

    let doc = Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Past 10 Minutes"))
        .add_view(&line_view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("On/Off")
        .add_bottom_axis_label("Past 10 Minutes")
        .to_svg_document()
        .unwrap();

    return Response::from_string(doc)
        .with_header("Content-type: image/svg+xml".parse::<Header>().unwrap())
        .with_header("Cache-Control: no-cache, must-revalidate, no-store".parse::<Header>().unwrap());
}

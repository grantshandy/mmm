use charts::{Chart, Color, LineSeriesView, ScaleLinear};
use chrono::prelude::*;
use std::path::PathBuf;

pub fn gen_graph(path: &PathBuf, length: usize, width: usize, height: usize) -> String {
    // Get text of the CSV file on disk.
    let plaintext = fstream::read_text(path).unwrap();

    // A list that contains time basic time values and the state associated with them.
    let mut main_data: Vec<(f32, bool)> = Vec::new();

    // Some characteristics to build our graph.
    let width = width as isize;
    let height = height as isize;
    let (top, right, bottom, left) = (40, 40, 50, 40);
    let min_time = -(length as f32);
    let max_time = 0.0;
    let max_value: f32 = 70.0;
    let min_value: f32 = -70.0;
    let on_num: f32 = 50.0;
    let off_num: f32 = -50.0;

    // Iterate through the lines of the text file (skipping the first row that tells us what each one is).
    for line in plaintext.lines().skip(1) {
        // Split the line at the delimeter (,).
        let mut line = line.split(",");

        // Create a DateTime object from the first object in the list.
        let date = DateTime::parse_from_rfc3339(line.nth(0).unwrap())
            .unwrap()
            .time();

        // Create a boolean value from the last object in the list.
        let state = match line.last().unwrap() {
            "On" => true,
            "Off" => false,
            &_ => false,
        };

        // Create a NaiveTime object from now to compare the values in the list.
        let now = Utc::now().time();

        // If the time is not more than 10 minutes ago...
        if date.signed_duration_since(now).num_minutes() > min_time as i64 {
            // Get the number of minutes ago it was...
            let date = date.signed_duration_since(now).num_seconds() as f32 / 60.0;
            // Push it to the list.
            main_data.push((date, state));
        } else {
            // Else push it to the end of the list. This will create duplicate points but it will stop me from doing more work.
            main_data.push((min_time, state))
        }
    }

    // Create a list of points for the graph.
    let mut line_data: Vec<(f32, f32)> = Vec::new();

    // Iterate through the main dataset and enumerate to get the number in the list that we're currently in.
    'main_loop: for (num, (date, state)) in main_data.iter().enumerate() {
        // Get ownership of date and state.
        let date = *date;
        let state = *state;

        // If we're not in the last or beginning of the list and the one before is our own state (if we're a duplicate), skip.
        if num != main_data.len() - 1 && num != 0 && main_data.get(num - 1).unwrap().1 == state {
            continue 'main_loop;
        }

        // Create a coordinate based on our current state.
        let coord: (f32, f32) = match state {
            true => (date, on_num),
            false => (date, off_num),
        };

        // If we're not at the beginning of the list and the one before us is not our current state...
        if num != 0 && main_data.get(num - 1).unwrap().1 != state {
            // Create another coordinate that is the opposite state but at the same place in time as us.
            let other_coord: (f32, f32) = match state {
                true => (date, off_num),
                false => (date, on_num),
            };

            // Push the other coordinate to the list. This is the one that is on the "hill" that each change in state creates.
            line_data.push(other_coord);
        }

        // Push the original coordinate to the list.
        line_data.push(coord);
    }

    // If the list has more than 0 things in it...
    if main_data.len() != 0 {
        // And the last one is not at the end...
        if main_data.last().unwrap().0 != max_time {
            // Get the last one and push it to it's state at the very end of the graph.
            match main_data.last().unwrap().1 {
                true => line_data.push((max_time, on_num)),
                false => line_data.push((max_time, off_num)),
            }
        }
    }

    // Random boilerplate code to generate the graph from our dataset...
    let x = ScaleLinear::new()
        .set_domain(vec![min_time, max_time])
        .set_range(vec![0, width - left - right]);

    let y = ScaleLinear::new()
        .set_domain(vec![min_value, max_value])
        .set_range(vec![height - top - bottom, 0]);

    let on_off_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_visibility(false)
        .set_colors(Color::from_vec_of_hex_strings(vec!["#d8d4cf"]))
        .load_data(&line_data)
        .unwrap();

    let temperature_data = parse_temperature_data(min_time, &plaintext);

    let temperature_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_visibility(false)
        .set_colors(Color::from_vec_of_hex_strings(vec!["#072ff7"]))
        .load_data(&temperature_data)
        .unwrap();

    let humidity_data = parse_humidity_data(min_time, &plaintext);

    let humidity_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_visibility(false)
        .set_colors(Color::from_vec_of_hex_strings(vec!["#e02702"]))
        .load_data(&humidity_data)
        .unwrap();

    let title = match length {
        1 => "Minute".to_string(),
        _ => format!("{} Minutes", length),
    };

    let doc = Chart::new()
        .font_color("#d8d4cf")
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(format!("Status Over The {}", &title))
        .add_view(&on_off_view)
        .add_view(&temperature_view)
        .add_view(&humidity_view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("On/Off")
        .add_bottom_axis_label(format!("Past {}", title))
        .to_svg_document() // I created this method in my own fork of rustplotlib.
        .unwrap();

    return doc;
}

fn parse_temperature_data(min_time: f32, text: &str) -> Vec<(f32, f32)> {
    let mut data: Vec<(f32, f32)> = Vec::new();

    for line in text.lines().skip(1) {
        // Split the line at the delimeter (,).
        let mut line = line.split(",");

        // Create a DateTime object from the first object in the list.
        let date = DateTime::parse_from_rfc3339(line.nth(0).unwrap())
            .unwrap()
            .time();

        let temperature: f32 = line.nth(0).unwrap().parse().unwrap();

        // Create a NaiveTime object from now to compare the values in the list.
        let now = Utc::now().time();

        // If the time is not more than 10 minutes ago...
        if date.signed_duration_since(now).num_minutes() > min_time as i64 {
            // Get the number of minutes ago it was...
            let date = date.signed_duration_since(now).num_seconds() as f32 / 60.0;
            // Push it to the list.
            data.push((date, temperature));
        } else {
            // Else push it to the end of the list. This will create duplicate points but it will stop me from doing more work.
            data.push((min_time, temperature));
        }
    }

    // If the list has more than 0 things in it...
    if data.len() != 0 {
        // if the last one is not at the end...
        if data.last().unwrap().0 != 0.0 {
            // Get the last one and push it to it's state at the very end of the graph.
            let last_temp = data.last().unwrap().1;
            data.push((0.0, last_temp));
        }
    }

    return data;
}


fn parse_humidity_data(min_time: f32, text: &str) -> Vec<(f32, f32)> {
    let mut data: Vec<(f32, f32)> = Vec::new();

    for line in text.lines().skip(1) {
        // Split the line at the delimeter (,).
        let mut line = line.split(",");

        // Create a DateTime object from the first object in the list.
        let date = DateTime::parse_from_rfc3339(line.nth(0).unwrap())
            .unwrap()
            .time();

        let humidity: u8 = line.nth(1).unwrap().parse().unwrap();

        // Create a NaiveTime object from now to compare the values in the list.
        let now = Utc::now().time();

        // If the time is not more than 10 minutes ago...
        if date.signed_duration_since(now).num_minutes() > min_time as i64 {
            // Get the number of minutes ago it was...
            let date = date.signed_duration_since(now).num_seconds() as f32 / 60.0;
            // Push it to the list.
            data.push((date, humidity as f32));
        } else {
            // Else push it to the end of the list. This will create duplicate points but it will stop me from doing more work.
            data.push((min_time, humidity as f32));
        }
    }

    // If the list has more than 0 things in it...
    if data.len() != 0 {
        // if the last one is not at the end...
        if data.last().unwrap().0 != 0.0 {
            // Get the last one and push it to it's state at the very end of the graph.
            let last_temp = data.last().unwrap().1;
            data.push((0.0, last_temp));
        }
    }

    return data;
}
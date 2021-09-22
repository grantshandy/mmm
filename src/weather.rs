use serde_json::Value;

pub struct Weather {
    pub temperature: f64,
    pub description: String,
    pub humidity: f64,
}

impl Weather {
    pub fn now() -> Self {
        let resp = match ureq::get(&format!(
            "http://api.weatherapi.com/v1/current.json?key={}&q=millcreek&aqi=no",
            include_str!("../weather_key")
        ))
        .call()
        {
            Ok(data) => data,
            Err(error) => {
                return Self {
                    temperature: 0.0,
                    description: format!("Error getting weather: {}", error),
                    humidity: 0.0,
                }
            }
        }
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

        let humidity = match &resp["current"]["humidity"] {
            Value::Number(desc) => match desc.as_f64() {
                Some(data) => data,
                None => panic!("not f64!!"),
            },
            _ => panic!("no humidity!"),
        };

        return Self {
            temperature,
            description,
            humidity,
        };
    }
}

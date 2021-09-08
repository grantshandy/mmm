use serde_json::Value;

pub struct Weather {
    pub temperature: f64,
    pub description: String,
}

impl Weather {
    pub async fn now() -> Self {
        let resp = surf::get("http://api.weatherapi.com/v1/current.json?key=c32481afef8d4a958ed134741210809&q=millcreek&aqi=no")
            .recv_json::<Value>()
            .await
            .unwrap();
    
        let description = match &resp["current"]["condition"]["text"] {
            Value::String(desc) => desc.clone(),
            _ => panic!("no text!"),
        };

        let temperature = match &resp["current"]["temp_c"] {
            Value::Number(desc) => match desc.as_f64() {
                Some(data) => data,
                None => panic!("not f64!!")
            },
            _ => panic!("no temp!"),
        };

        return Self {
            temperature,
            description,
        };  
    }
}
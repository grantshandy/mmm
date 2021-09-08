use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub temperature: String,
    pub wind: String,
    pub description: String,
}

impl Weather {
    pub async fn now() -> Self {
        let resp = surf::get("https://goweather.herokuapp.com/weather/millcreek")
            .recv_json::<Weather>()
            .await
            .unwrap();
    
        return resp;
    }
}
use chrono::prelude::*;
use system_config::Config;
use std::collections::HashMap;

pub struct Switcher {
    pub state: bool,
    pub config: Config,
}

impl Switcher {
    pub fn new() -> Self {
        let config = Config::new("mmm").unwrap();

        Self {
            state: false,
            config,
        }
    }

    pub fn on(&mut self) -> Result<(), (String, usize)> {
        match self.state {
            true => {
                println!("it was already on so I did nothing");
            }
            false => {
                println!("turning on...");
                self.state = true;
                self.config.write_insert(Utc::now().to_string(), true.to_string()).unwrap();
            }
        }

        return Ok(());
    }

    pub fn off(&mut self) -> Result<(), (String, usize)> {
        match self.state {
            false => {
                println!("it was already off so I did nothing");
            }
            true => {
                println!("turning off...");
                self.state = false;
                self.config.write_insert(Utc::now().to_string(), false.to_string()).unwrap();
            }
        }

        return Ok(());
    }
}

pub fn get_data() -> HashMap<String, bool> {
    let config = Config::new("mmm").unwrap();
    let plaintext = fstream::read_text(config.path).unwrap();
    let yaml: HashMap<String, String> = serde_yaml::from_str(&plaintext).unwrap();
    let mut result = HashMap::new();

    for (date, state) in yaml {
        let state = state.parse::<bool>().unwrap();

        result.insert(date, state);
    }

    return result;
}
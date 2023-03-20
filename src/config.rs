use serde_json::Value;
use std::fs::File;

#[derive(Debug)]
pub struct Config {
    pub discord_token: String,
    pub json: Value,
}

impl Config {
    pub fn load(path: impl AsRef<str>) -> Self {
        let file = match File::open(path.as_ref()) {
            Ok(v) => v,
            Err(e) => panic!("Error loading config: {}", e),
        };

        let json: Value = serde_json::from_reader(file).expect("Error loading config!");
        let discord_token = json
            .as_object()
            .unwrap()
            .get("discord_token")
            .unwrap()
            .as_str()
            .unwrap();

        Self {
            discord_token: discord_token.to_string(),
            json,
        }
    }
}

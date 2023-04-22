use serde::Deserialize;
use std::{fs::File, path::Path};

#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub discord_token: String,
    pub openai_key: String,
}

impl GlobalConfig {
    pub fn load<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let file = match File::open(path) {
            Ok(v) => v,
            Err(e) => panic!("Error loading config: {}", e),
        };

        let json: GlobalConfig = serde_json::from_reader(file).expect("Error loading config!");

        json
    }
}

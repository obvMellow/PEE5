use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs::File, path::Path};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum Plugins {
    Afk,
    Automod,
    Chat,
    Logging,
    Xp,
}

pub trait IsPlugin {
    fn afk(&self) -> bool;
    fn automod(&self) -> bool;
    fn chat(&self) -> bool;
    fn logging(&self) -> bool;
    fn xp(&self) -> bool;
}

impl IsPlugin for Vec<Plugins> {
    /// Shorthand for checking if afk plugin is enabled.
    fn afk(&self) -> bool {
        self.contains(&Plugins::Afk)
    }

    /// Shorthand for checking if automod plugin is enabled.
    fn automod(&self) -> bool {
        self.contains(&Plugins::Automod)
    }

    /// Shorthand for checking if chat plugin is enabled.
    fn chat(&self) -> bool {
        self.contains(&Plugins::Chat)
    }

    /// Shorthand for checking if logging plugin is enabled.
    fn logging(&self) -> bool {
        self.contains(&Plugins::Logging)
    }

    /// Shorthand for checking if xp plugin is enabled.
    fn xp(&self) -> bool {
        self.contains(&Plugins::Xp)
    }
}

impl Plugins {
    pub fn to_str(&self) -> &str {
        match self {
            Plugins::Afk => "Afk",
            Plugins::Automod => "Automod",
            Plugins::Chat => "Chat",
            Plugins::Logging => "Logging",
            Plugins::Xp => "Xp",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildConfig {
    automod: bool,
    blacklisted_words: Vec<String>,
    id: u64,
    log_channel_id: Option<u64>,
    users: HashMap<u64, usize>,
    afk: HashMap<u64, String>,
    plugins: Vec<Plugins>,
}

impl GuildConfig {
    pub fn new(id: impl Into<u64>) -> Self {
        GuildConfig {
            automod: false,
            blacklisted_words: Vec::new(),
            id: id.into(),
            log_channel_id: None,
            users: HashMap::new(),
            afk: HashMap::new(),
            plugins: Vec::new(),
        }
    }

    pub fn from_reader<R>(reader: R) -> Result<Self, serde_json::Error>
    where
        R: std::io::Read,
    {
        serde_json::from_reader(reader)
    }

    pub fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(slice)
    }

    pub fn from_value(json: Value) -> Self {
        serde_json::from_value(json).unwrap()
    }

    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }

    pub fn to_writer<W>(&self, writer: W) -> Result<(), serde_json::Error>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, &self)
    }

    pub fn to_writer_pretty<W>(&self, writer: W) -> Result<(), serde_json::Error>
    where
        W: std::io::Write,
    {
        serde_json::to_writer_pretty(writer, &self)
    }

    pub fn save<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let file = match File::create(path) {
            Ok(v) => v,
            Err(e) => return Err(Error::Io(e)),
        };

        match self.to_writer_pretty(file) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Json(e)),
        }
    }

    pub fn get_automod(&self) -> bool {
        self.automod
    }

    pub fn get_automod_mut(&mut self) -> &mut bool {
        &mut self.automod
    }

    pub fn get_blacklisted_words(&self) -> &Vec<String> {
        &self.blacklisted_words
    }

    pub fn get_blacklisted_words_mut(&mut self) -> &mut Vec<String> {
        &mut self.blacklisted_words
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_log_channel_id(&self) -> Option<u64> {
        self.log_channel_id
    }

    pub fn get_log_channel_id_mut(&mut self) -> &mut Option<u64> {
        &mut self.log_channel_id
    }

    pub fn get_users(&self) -> &HashMap<u64, usize> {
        &self.users
    }

    pub fn get_users_mut(&mut self) -> &mut HashMap<u64, usize> {
        &mut self.users
    }

    pub fn get_afk(&self) -> &HashMap<u64, String> {
        &self.afk
    }

    pub fn get_afk_mut(&mut self) -> &mut HashMap<u64, String> {
        &mut self.afk
    }

    pub fn get_plugins(&self) -> &Vec<Plugins> {
        &self.plugins
    }

    pub fn get_plugins_mut(&mut self) -> &mut Vec<Plugins> {
        &mut self.plugins
    }
}

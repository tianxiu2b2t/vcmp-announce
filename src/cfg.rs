use std::{collections::HashMap, str::FromStr, sync::OnceLock};

use serde::{Serialize, Deserialize};
use tracing::Level;

pub const DEFAULT_MASTERS: &[&str] = &[
    "http://master.vc-mp.org",
    "http://master.thijn.ovh",
    "http://master.adtec.ovh",
];

#[derive(Debug, Clone)]
pub struct LogLevel(Level);

impl LogLevel {
    pub fn get_value(&self) -> Level {
        self.0
    }
}

impl Serialize for LogLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.get_value().as_str())
    }
}


impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(Level::from_str(&s).map_err(serde::de::Error::custom)?))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub servers: HashMap<String, Server>,
    /// 按 second 计算
    pub interval: Option<u64>,
    /// 发送的 url，默认就
    pub announce_masters: Option<Vec<String>>,
    /// log level
    pub log_level: Option<LogLevel>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            servers: HashMap::from([("default".to_string(), Server { port: 8192 })]),
            interval: Some(60),
            announce_masters: Some(DEFAULT_MASTERS.iter().map(|f| f.to_string()).collect()),
            log_level: Some(LogLevel(Level::INFO)),
        }
    }
}

impl Config {
    pub fn announce_masters(&self) -> Vec<String> {
        match &self.announce_masters {
            Some(v) => v.clone(),
            None => DEFAULT_MASTERS.iter().map(|f| f.to_string()).collect(),
        }
    }

    pub fn interval(&self) -> u64 {
        self.interval.unwrap_or(60)
    }

    pub fn interval_as_duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.interval())
    }

    pub fn log_level(&self) -> Level {
        self.log_level.clone().map(|f| f.get_value()).unwrap_or(Level::INFO)
    }
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();
pub fn load_config(path: &str) {
    let config = std::fs::read_to_string(path).expect("no config file");
    let config = toml::from_str::<Config>(&config).expect("invalid config file");
    CONFIG.set(config).unwrap();
}

pub fn get_announce_masters() -> Vec<String> {
    let masters = match CONFIG.get() {
        Some(v) => v.announce_masters(),
        None => DEFAULT_MASTERS.iter().map(|f| f.to_string()).collect(),
    };
    masters.iter().map(|f| format!("{}/announce.php", f.trim_end_matches("/"))).collect()
}

pub fn get_level_log() -> Level {
    match CONFIG.get() {
        Some(v) => v.log_level(),
        None => Level::INFO,
    }
}

pub fn get_servers() -> Option<HashMap<String, Server>> {
    // if hashmap is empty, return None else 
    match CONFIG.get() {
        Some(v) => {
            let servers = v.servers.clone();
            if servers.is_empty() {
                None
            }
            else {
                Some(servers)
            }
        },
        None => None,
    }
}

pub fn get_interval() -> u64 {
    match CONFIG.get() {
        Some(v) => v.interval(),
        None => 60,
    }
}
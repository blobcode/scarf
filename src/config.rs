use serde::Deserialize;
use std::fs;

// main config struct
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub ip: String,
    pub port: i32,
    pub method: Option<String>,
    pub service: Option<Vec<ServiceConfig>>,
}

// struct for individual service config
#[derive(Debug, Deserialize, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub address: String,
    pub weight: Option<i32>,
}

// load in config file (toml)
pub fn load(path: String) -> Config {
    let file = fs::read_to_string("./scarf.toml").expect("Unable to read file");
    let config: Config = toml::from_str(&file).unwrap();
    return config;
}

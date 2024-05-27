use std::fs;
use serde_json::{from_str};
use crate::models;

pub fn load_config() -> models::Config {
    let config_file: String = fs::read_to_string("./src/config/config.json")
        .expect("Should have been able to read the file");

    return from_str(&*config_file).expect("JSON was not well-formatted");
}
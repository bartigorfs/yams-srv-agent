use std::str::FromStr;
use std::string::ToString;
use ascii::AsciiString;
use tiny_http::{Header, HeaderField};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub mod process;
pub mod disks;
pub mod networks;
pub mod components;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) host: String,
    pub(crate) port: String,
}

fn create_ascii_string(s: String) -> AsciiString {
    match AsciiString::from_str(&*s) {
        Ok(s) => s,
        Err(_) => panic!("Failed to create AsciiString"),
    }
}

pub static JSON_HEADER: Lazy<Header> = Lazy::new(|| Header {
    field: HeaderField::from_str("Content-Type").unwrap(),
    value: create_ascii_string("application/json".to_string()),
});
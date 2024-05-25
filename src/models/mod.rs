use std::str::FromStr;
use std::string::ToString;
use ascii::AsciiString;
use tiny_http::{Header, HeaderField};
use once_cell::sync::Lazy;
use serde::{Serialize};

pub mod process;
pub mod disks;
pub mod networks;

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
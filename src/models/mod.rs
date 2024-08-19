use std::str::FromStr;
use ascii::AsciiString;
use serde::{Deserialize};

pub mod process;
pub mod disks;
pub mod networks;
pub mod components;
pub mod app;
pub mod generic_info;

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
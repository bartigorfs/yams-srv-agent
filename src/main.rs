use std::fmt::format;
use tiny_http::{Server};
use chrono::prelude::*;
use sysinfo::System;

mod logger;
mod models;
mod handler;
mod middleware;
mod config;

fn main() {
    let config: models::Config = config::load_config();

    let server: Server = Server::http(format!("{}:{}", config.host, config.port)).unwrap();
    let message = format!("{tz} Server started on {}", config.port, tz = Utc::now());
    //logger::write_to_log(message.clone()); #Later homie
    println!("{}", message);

    let mut sys = System::new_all();
    sys.refresh_all();

    for request in server.incoming_requests() {
        handler::handle_status_request(request);
    }
}

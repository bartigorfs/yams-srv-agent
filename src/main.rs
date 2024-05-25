use tiny_http::{Server};
use chrono::prelude::*;
use sysinfo::System;

mod logger;
mod models;
mod handler;
mod middleware;
mod config;

fn main() {
    let server: Server = Server::http("0.0.0.0:8000").unwrap();
    let message = format!("{tz} Server started on {port}", port = 8000, tz = Utc::now());
    //logger::write_to_log(message.clone()); #Later homie
    println!("{}", message);

    let mut sys = System::new_all();
    sys.refresh_all();

    for request in server.incoming_requests() {
        handler::handle_status_request(request);
    }
}

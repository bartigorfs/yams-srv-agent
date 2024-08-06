use std::env;
use std::net::SocketAddr;
use chrono::prelude::*;
use dotenv::dotenv;
use lazy_static::lazy_static;
use tokio::net::TcpListener;
use tokio::sync::{watch, Mutex};

use crate::models::app::AppConfig;
use crate::util::graceful_util::get_graceful_signal;

mod logger;
mod models;
mod handler;
mod middleware;
mod util;

lazy_static! {
    static ref APP_CONFIG: AppConfig = {
        dotenv().ok();

        let app_port: u16 = env::var("PORT")
            .expect("PORT must be set.")
            .parse()
            .unwrap();

        let host: String = env::var("HOST").expect("HOST must be set.");

        let host_array: Vec<u16> = host
            .split(".")
            .map(|s| s.parse::<u16>().unwrap_or(0))
            .collect::<Vec<u16>>();

         AppConfig {
            host: host_array,
            port: app_port,
        }
    };
}

pub async fn get_app_config() -> &'static AppConfig {
    &APP_CONFIG
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    let config: &AppConfig = get_app_config().await;

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let listener: TcpListener = TcpListener::bind(addr).await?;

    let (shutdown_tx, mut shutdown_rx) = watch::channel(());
    let shutdown_signal = get_graceful_signal(shutdown_tx);

    let message = format!("{tz} Server started on {}", config.port, tz = Utc::now());
    println!("{}", message);

    tokio::select! {
        _ = shutdown_signal => {
            println!("Received shutdown signal");
        }
        _ = run_server(listener, pool.clone(), &mut shutdown_rx) => {
            println!("Server exited");
        }
    }

    Ok(())
}

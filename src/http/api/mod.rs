use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tower::ServiceBuilder;

use crate::get_app_config;
use crate::http::handlers::router::router;
use crate::http::middleware::logger::Logger;
use crate::http::middleware::query_params::QueryParams;
use crate::models::app::AppConfig;

pub async fn run_server(
    listener: TcpListener,
    shutdown_rx: &mut tokio::sync::watch::Receiver<()>,
) {
    loop {
        tokio::select! {
            Ok((stream, addr)) = listener.accept() => {
                let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
           let config: &AppConfig = get_app_config().await;

           let svc = service_fn(router);

            let svc = ServiceBuilder::new().layer_fn(Logger::new).service(svc);
            // let svc = ServiceBuilder::new().layer_fn(|inner| OriginValidation::new(inner, addr.to_string(), Arc::clone(&config.trusted_origins))).service(svc);
            // let svc = ServiceBuilder::new().layer_fn(|inner| TOTPCheck::new(inner, config.totp_secret.clone())).service(svc);
            let svc = ServiceBuilder::new().layer_fn(QueryParams::new).service(svc);

            if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
            }
            _ = shutdown_rx.changed() => {
                println!("Shutdown signal received, stopping server.");
                break;
            }
        }
    }
}
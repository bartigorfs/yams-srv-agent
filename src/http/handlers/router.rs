use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Method, Request, Response, StatusCode};
use crate::http::handlers::status::status_handler;
use crate::http::handlers::system_info::system_info_handler;
use crate::util::hyper_util::{empty, send_empty_ok};

pub async fn router(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/health") => send_empty_ok(),

        (&Method::GET, "/system-info") => system_info_handler(req).await,

        (&Method::GET, "/status") => status_handler(req).await,

        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
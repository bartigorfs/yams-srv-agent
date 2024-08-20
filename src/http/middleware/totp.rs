use std::time::{SystemTime, UNIX_EPOCH};

use bytes::Bytes;
use futures::future::{BoxFuture, FutureExt};
use http_body_util::combinators::BoxBody;
use hyper::{body::Incoming, Request, Response, StatusCode};
use hyper::Error;
use hyper::http::HeaderValue;
use hyper::service::Service;
use otpauth::TOTP;

use crate::util::hyper_util::empty;

#[derive(Debug, Clone)]
pub struct TOTPCheck<S> {
    inner: S,
    totp_key: String,
}

impl<S> TOTPCheck<S> {
    pub fn new(inner: S, totp_key: String) -> Self {
        TOTPCheck {
            inner,
            totp_key,
        }
    }
}

type Req = Request<Incoming>;

impl<S> Service<Req> for TOTPCheck<S>
where
    S: Service<Req, Response=Response<BoxBody<Bytes, Error>>, Error=Error> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: Req) -> Self::Future {
        fn get_unauthorized_resp() -> Response<BoxBody<Bytes, Error>> {
            let mut resp = Response::new(empty());
            *resp.status_mut() = StatusCode::UNAUTHORIZED;
            resp
        }

        match req.headers().get("Authorization").and_then(|header: &HeaderValue| header.to_str().ok()) {
            Some(totp_token_str) => match totp_token_str.parse::<u32>() {
                Ok(totp_token) => {
                    let secret: String = self.totp_key.parse().unwrap();
                    let totp: TOTP = TOTP::new(secret);

                    let current_timestamp: u64 = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs();

                    let period: u64 = 30;

                    if totp.verify(totp_token, period, current_timestamp) {
                        let fut = self.inner.call(req);
                        fut.boxed()
                    } else {
                        futures::future::ok(get_unauthorized_resp()).boxed()
                    }
                }
                Err(_) => {
                    futures::future::ok(get_unauthorized_resp()).boxed()
                }
            },
            None => {
                futures::future::ok(get_unauthorized_resp()).boxed()
            }
        }
    }
}
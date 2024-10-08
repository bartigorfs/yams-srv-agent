use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response, StatusCode};
use serde_json::json;
use sysinfo::System;
use crate::util::hyper_util::send_json_response;

pub async fn system_info_handler() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let response = json!({
        "system": {
        "name": System::name().unwrap(),
        "kernel_version": System::kernel_version().unwrap(),
        "os_version": System::os_version().unwrap(),
        "host_name": System::host_name().unwrap(),
        },
    });

    send_json_response(response, StatusCode::OK)
}
use std::collections::HashMap;
use std::ffi::OsString;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response, StatusCode};
use serde_json::{json, Value};
use sysinfo::{Components, Disks, Networks, RefreshKind, System};
use crate::http::service::collect::{collect_component_info, collect_disk_info, collect_memory_info, collect_network_info, collect_process_info};
use crate::models;
use crate::models::components::Component;
use crate::models::disks::DiskInner;
use crate::models::generic_info::GenericInfo;
use crate::models::networks::NetworksInner;
use crate::models::process::ProcessInfo;
use crate::util::hyper_util::send_json_response;

pub async fn status_handler(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let mut sys: System = System::new_all();
    sys.refresh_all();

    let disks_vec: Vec<DiskInner> = collect_disk_info(&sys);
    let components_vec: Vec<Component> = collect_component_info(&sys);
    let generic_info: GenericInfo = collect_memory_info(&sys);

    let process_info_vec: Vec<ProcessInfo> = collect_process_info(&mut sys).await;

    let mut sys_info: Value = json!({
        "memory": {
            "total": generic_info.total,
            "used": generic_info.used,
            "total_swap": generic_info.total_swap,
            "used_swap": generic_info.used_swap
        },
        "cpu": {
            "amount": generic_info.cpu_amount
        },
        "processes": process_info_vec,
        "disks": disks_vec,
        "components": components_vec,
    });

    if let Some(params) = req.extensions().get::<HashMap<String, String>>() {
        if let Some(network) = params.get("include") {
            if network == "network" {
                sys_info["networks"] = serde_json::to_value(collect_network_info(&sys)).unwrap_or_else(|_| json!(null));
            }
        }
    }

    send_json_response(sys_info, StatusCode::OK)
}
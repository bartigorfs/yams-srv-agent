use std::ffi::OsString;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response, StatusCode};
use serde_json::{json, Value};
use sysinfo::{Components, Disks, Networks, System};
use crate::models;
use crate::util::hyper_util::send_json_response;

pub async fn status_handler(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let mut sys: System = System::new_all();

    sys.refresh_all();

    let mut process_info_vec: Vec<models::process::ProcessInfo> = vec![];

    //TODO: Replace with converter
    for (.., process) in sys.processes() {
        process_info_vec.push(models::process::ProcessInfo {
            name: process.name().to_string_lossy().parse().unwrap(),
            disk_usage: models::process::DiskUsage {
                total_written_bytes: process.disk_usage().total_written_bytes,
                written_bytes: process.disk_usage().written_bytes,
                total_read_bytes: process.disk_usage().total_read_bytes,
                read_bytes: process.disk_usage().read_bytes,
            },
            cpu_usage: process.cpu_usage(),
            mem_usage: process.memory(),
            virt_mem_usage: process.virtual_memory(),
            proc_path: process.exe(),
            parent_pid: process.parent().map(|p| p.as_u32()),
            workdir: process.root(),
            run_time: process.run_time(),
            start_time: process.start_time(),
            status: models::process::ProcessStatus::from(process.status()),
        });
    }

    //TODO: Replace with converter
    let disks: Disks = Disks::new_with_refreshed_list();
    let mut disks_vec: Vec<models::disks::DiskInner> = vec![];

    for disk in &disks {
        disks_vec.push(models::disks::DiskInner {
            type_: models::disks::DiskKind::from(disk.kind()),
            name: OsString::from(disk.name()),
            file_system: OsString::from(disk.file_system()),
            mount_point: disk.mount_point().to_path_buf(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            is_removable: disk.is_removable(),
        });
    }

    let networks: Networks = Networks::new_with_refreshed_list();
    let mut networks_vec: Vec<models::networks::NetworksInner> = vec![];

    for (idk, net_data) in &networks {
        networks_vec.push(models::networks::NetworksInner {
            name: idk.to_string(),
            data: models::networks::NetworkData {
                inner: models::networks::NetworkDataInner {
                    total_received: net_data.total_received(),
                    received: net_data.received(),
                    packets_received: net_data.packets_received(),
                    errors_on_received: net_data.errors_on_received(),
                    packets_transmitted: net_data.packets_transmitted(),
                    total_errors_on_transmitted: net_data.total_errors_on_transmitted(),
                    total_transmitted: net_data.total_transmitted(),
                    transmitted: net_data.transmitted(),
                    errors_on_transmitted: net_data.errors_on_transmitted(),
                    total_errors_on_received: net_data.total_errors_on_received(),
                    total_packets_received: net_data.total_packets_received(),
                    total_packets_transmitted: net_data.total_packets_transmitted(),
                },
            },
        });
    }

    //TODO: Replace with converter
    let components: Components = Components::new_with_refreshed_list();
    let mut components_vec: Vec<models::components::Component> = vec![];

    for (component) in &components {
        components_vec.push(models::components::Component {
            temperature: component.temperature(),
            max: component.max(),
            critical: component.critical(),
            label: component.label().parse().unwrap(),
        });
    }

    let sys_info: Value = json!({
        "memory": {
            "total": sys.total_memory(),
            "used": sys.used_memory(),
            "total_swap": sys.total_swap(),
            "used_swap": sys.used_swap()
        },
        "cpu": {
            "amount": sys.cpus().len()
        },
        "processes": process_info_vec,
        "disks": disks_vec,
        "networks": networks_vec,
        "components": components_vec,
    });

    send_json_response(sys_info, StatusCode::OK)
}
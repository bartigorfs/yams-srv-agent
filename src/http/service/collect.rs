use std::ffi::OsString;
use sysinfo::{Components, Disks, Networks, ProcessesToUpdate, ProcessRefreshKind, System};
use crate::models;
use crate::models::generic_info::{GenericInfo};

pub async fn collect_process_info(sys: &mut System) -> Vec<models::process::ProcessInfo> {
    tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;

    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        ProcessRefreshKind::new().with_cpu().with_disk_usage().with_memory(),
    );

    let mut process_info_vec: Vec<models::process::ProcessInfo> = Vec::new();

    for (_, process) in sys.processes() {
        let disk_usage = process.disk_usage();

        process_info_vec.push(models::process::ProcessInfo {
            name: process.name().to_string_lossy().parse().unwrap(),
            disk_usage: models::process::DiskUsage {
                total_written_bytes: disk_usage.total_written_bytes,
                written_bytes: disk_usage.written_bytes,
                total_read_bytes: disk_usage.total_read_bytes,
                read_bytes: disk_usage.read_bytes,
            },
            pid: usize::from(process.pid()),
            cpu_usage: process.cpu_usage(),
            mem_usage: process.memory(),
            virt_mem_usage: process.virtual_memory(),
            proc_path: process.exe(),
            parent_pid: process.parent().map(|p| p.as_u32()),
            workdir: process.root(),
            run_time: process.run_time(),
            start_time: process.start_time(),
            status: process.status().into(),
        });
    }

    process_info_vec
}
// Функция для сбора информации о дисках
pub fn collect_disk_info(sys: &System) -> Vec<models::disks::DiskInner> {
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

    disks_vec
}

// Функция для сбора информации о сетях
pub fn collect_network_info(sys: &System) -> Vec<models::networks::NetworksInner> {
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

    networks_vec
}

// Функция для сбора информации о компонентах
pub fn collect_component_info(sys: &System) -> Vec<models::components::Component> {
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

    components_vec
}

pub fn collect_memory_info(sys: &System) -> GenericInfo {
    GenericInfo {
        total: sys.total_memory(),
        used: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.total_swap(),
        cpu_amount: sys.cpus().len(),
    }
}
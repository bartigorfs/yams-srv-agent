use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sysinfo::{Components, Disks, Networks, Pid, Process, ProcessStatus, System};
use tiny_http::{Request, Response};
use crate::{logger, models};

// fn convert_sys_process_to_model(sys_proc: &HashMap<Pid, Process>) -> HashMap<models::Pid, models::Process> {
//     sys_proc.iter().map(|(&pid, proc)| {
//         (
//             models::Pid(pid.as_u32() as i32),
//             models::Process {
//                 inner: models::ProcessInner {
//                     name: proc.name().to_string(),
//                     cmd: Vec::from(proc.cmd().clone()),
//                     exe: proc.exe().cloned(),
//                     pid: models::Pid(proc.pid().as_u32() as i32),
//                     user_id: proc.user_id(),
//                     environ: proc.environ().clone(),
//                     cwd: proc.cwd().cloned(),
//                     root: proc.root().cloned(),
//                     memory: proc.memory(),
//                     virtual_memory: proc.virtual_memory(),
//                     parent: proc.parent().map(|p| models::Pid(p.as_u32() as i32)),
//                     status: match proc.status() {
//                         ProcessStatus::Idle => models::ProcessStatus::Idle,
//                         ProcessStatus::Run => models::ProcessStatus::Run,
//                         ProcessStatus::Sleep => models::ProcessStatus::Sleep,
//                         ProcessStatus::Stop => models::ProcessStatus::Stop,
//                         ProcessStatus::Zombie => models::ProcessStatus::Zombie,
//                         ProcessStatus::Tracing => models::ProcessStatus::Tracing,
//                         ProcessStatus::Dead => models::ProcessStatus::Dead,
//                         ProcessStatus::Wakekill => models::ProcessStatus::Wakekill,
//                         ProcessStatus::Waking => models::ProcessStatus::Waking,
//                         ProcessStatus::Parked => models::ProcessStatus::Parked,
//                         ProcessStatus::LockBlocked => models::ProcessStatus::LockBlocked,
//                         ProcessStatus::UninterruptibleDiskSleep => models::ProcessStatus::UninterruptibleDiskSleep,
//                         ProcessStatus::Unknown(code) => models::ProcessStatus::Unknown(code),
//                     },
//                     handle: proc.handle().map(|h| Arc::new(models::HandleWrapper(h.0 as usize))),
//                     cpu_calc_values: models::CPUsageCalculationValues {
//                         old_process_sys_cpu: proc.cpu_calc_values().old_process_sys_cpu,
//                         old_process_user_cpu: proc.cpu_calc_values().old_process_user_cpu,
//                         old_system_sys_cpu: proc.cpu_calc_values().old_system_sys_cpu,
//                         old_system_user_cpu: proc.cpu_calc_values().old_system_user_cpu,
//                     },
//                     start_time: proc.start_time(),
//                     run_time: proc.run_time(),
//                     cpu_usage: proc.cpu_usage(),
//                     updated: proc.updated(),
//                     old_read_bytes: proc.old_read_bytes(),
//                     old_written_bytes: proc.old_written_bytes(),
//                     read_bytes: proc.read_bytes(),
//                     written_bytes: proc.written_bytes(),
//                 }
//             }
//         )
//     }).collect()
// }
pub fn handle_status_request(request: Request) {
    let utc: DateTime<Utc> = Utc::now();

    let message = format!("{:?} method: {:?}, url: {:?}, headers: {:?}",
                          utc,
                          request.method(),
                          request.url(),
                          request.headers()
    );

    // logger::write_to_log(message);

    let mut sys = System::new_all();

    sys.refresh_all();


    //let sys_proc= convert_sys_process_to_model(sys.processes());
    let mut process_info_vec: Vec<models::process::ProcessInfo> = vec![];

    for (pid, process) in sys.processes() {
        let process_info: models::process::ProcessInfo = models::process::ProcessInfo {
            name: process.name().parse().unwrap(),
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
            parent_pid: process.parent().map(|p| Pid(p.as_u32() as i32 as usize)),
            workdir: process.root(),
            run_time: process.run_time(),
            start_time: process.start_time(),
            status: models::process::ProcessStatus::from(process.status()),
        };
        process_info_vec.push(process_info);
        // println!("[{pid}] Name: {}  DiskUsage: {:?},  CpuUsage: {:?},  MemUsage: {:?} VirtMemUsage: {:?}, UID: {:?} PATH: {:?} PARENT: {:?} ROOT: {:?} RunTime: {:?} StartTime: {:?} Status: {:?} ",
        //          process.name(),
        //          process.disk_usage(),
        //          process.cpu_usage(),
        //          process.memory(),
        //          process.virtual_memory(),
        //          process.user_id(),
        //          process.exe(),
        //          process.parent(),
        //          process.root(),
        //          process.run_time(),
        //          process.start_time(),
        //          process.status()
        // );
    }


    let sys_info = json!({
        "system": {
            "name": System::name().unwrap(),
            "kernel_version": System::kernel_version().unwrap(),
            "os_version": System::os_version().unwrap(),
            "host_name": System::host_name().unwrap(),
        },
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
        //"disks": Disks::new_with_refreshed_list(),
        //"networks": Networks::new_with_refreshed_list(),
        //"components": Components::new_with_refreshed_list(),
    });

    sys.processes();

    let mut response = Response::from_string(sys_info.to_string());
    response.add_header(models::JSON_HEADER.clone());

    request.respond(response).expect("TODO: panic message");
}
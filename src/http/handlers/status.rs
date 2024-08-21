use std::fmt::format;
use crate::util::hyper_util::{send_json_error_response, send_json_response};
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response, StatusCode};
use serde_json::{json, Value};
use std::fs::File;
use std::io::{self, BufRead};
use std::thread;
use crate::models::cpu::SingleCpu;

pub async fn status_handler(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let path = "/proc/stat";
    let mut total_usage: f64 = 0.0;
    let num_cpus = thread::available_parallelism().unwrap().get();
    let mut load_by_core: Vec<SingleCpu> = vec![];

    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Ошибка при открытии файла: {}", err);
            return send_json_error_response("Cannot acquire status", StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.starts_with("cpu ") {
                    let values: Vec<&str> = line.split_whitespace().collect();
                    if values.len() >= 8 {
                        total_usage = calculate_cpu_usage(&values);
                    }
                    continue;
                }
                for core_num in 0..num_cpus - 1 {
                    let cpu_name_str = format!("cpu{}", core_num);
                    println!("{}", cpu_name_str);
                    let cpu_name = cpu_name_str.as_str();

                    if line.starts_with(cpu_name) {
                        let values: Vec<&str> = line.split_whitespace().collect();
                        let usage = calculate_cpu_usage(&values);

                        load_by_core.push(SingleCpu {
                            name: cpu_name.parse().unwrap(),
                            load: usage,
                        })
                    }
                }
                // break;
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    let sys_info: Value = json!({
        "num_cpu": num_cpus,
        "cpu_total_usage": total_usage,
        "load_by_cores": load_by_core
    });

    // let mut sys_info: Value = json!({
    //     "memory": {
    //         "total": generic_info.total,
    //         "used": generic_info.used,
    //         "total_swap": generic_info.total_swap,
    //         "used_swap": generic_info.used_swap
    //     },
    //     "processes": process_info_vec,
    //     "disks": disks_vec,
    //     "components": components_vec,
    // });

    send_json_response(sys_info, StatusCode::OK)
    // send_json_error_response("Cannot acquire status", StatusCode::INTERNAL_SERVER_ERROR)
}

fn calculate_cpu_usage(values: &[&str]) -> f64 {
    let user: f64 = values[1].parse().unwrap_or(0.0);
    let nice: f64 = values[2].parse().unwrap_or(0.0);
    let system: f64 = values[3].parse().unwrap_or(0.0);
    let idle: f64 = values[4].parse().unwrap_or(0.0);
    let iowait: f64 = values[5].parse().unwrap_or(0.0);
    let irq: f64 = values[6].parse().unwrap_or(0.0);
    let softirq: f64 = values[7].parse().unwrap_or(0.0);
    let total = user + nice + system + idle + iowait + irq + softirq;
    let usage = (total - idle) / total * 100.0;
    usage
}
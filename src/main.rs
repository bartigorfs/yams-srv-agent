use tiny_http::{Server, Response};
use chrono::prelude::*;
use serde_json::{json, Value};
use sysinfo::{
    Components, Disks, Networks, System,
};

mod logger;
mod models;

fn main() {
    let server: Server = Server::http("0.0.0.0:8000").unwrap();
    let message = format!("Server started on {}", port = 8000);
    logger::write_to_log(message.clone());
    println!("{}", message);

    for request in server.incoming_requests() {
        let utc: DateTime<Utc> = Utc::now();

        println!("{:?} method: {:?}, url: {:?}, headers: {:?}",
                 utc,
                 request.method(),
                 request.url(),
                 request.headers()
        );

// Please note that we use "new_all" to ensure that all list of
// components, network interfaces, disks and users are already
// filled!
        let mut sys = System::new_all();

// First we update all information of our `System` struct.
        sys.refresh_all();

        println!("=> system:");
// RAM and swap information:
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());
        println!("total swap  : {} bytes", sys.total_swap());
        println!("used swap   : {} bytes", sys.used_swap());

// Display system information:
        println!("System name:             {:?}", System::name().unwrap());
        println!("System kernel version:   {:?}", System::kernel_version());
        println!("System OS version:       {:?}", System::os_version());
        println!("System host name:        {:?}", System::host_name());

// Number of CPUs:
        println!("NB CPUs: {}", sys.cpus().len());

// Display processes ID, name na disk usage:
        for (pid, process) in sys.processes() {
            println!("[{pid}] {} {:?}", process.name(), process.disk_usage());
        }

// We display all disks' information:
        println!("=> disks:");
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            println!("{disk:?}");
        }

// Network interfaces name, total data received and total data transmitted:
        let networks = Networks::new_with_refreshed_list();
        println!("=> networks:");
        for (interface_name, data) in &networks {
            println!(
                "{interface_name}: {} B (down) / {} B (up)",
                data.total_received(),
                data.total_transmitted(),
            );
            // If you want the amount of data received/transmitted since last call
            // to `Networks::refresh`, use `received`/`transmitted`.
        }

// Components temperature:
        let components = Components::new_with_refreshed_list();
        println!("=> components:");
        for component in &components {
            println!("{component:?}");
        }

        let resp: Value = json!({"boo": true});

        let mut response = Response::from_string(resp.to_string());
        response.add_header(models::JSON_HEADER.clone());

        request.respond(response).expect("TODO: panic message");
    }
}

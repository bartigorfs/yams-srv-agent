pub mod process;

use std::path::{Path};
use std::str::FromStr;
use std::string::ToString;
use ascii::AsciiString;
use tiny_http::{Header, HeaderField};
use once_cell::sync::Lazy;
use serde::{Serialize};

fn create_ascii_string(s: String) -> AsciiString {
    match AsciiString::from_str(&*s) {
        Ok(s) => s,
        Err(_) => panic!("Failed to create AsciiString"),
    }
}

pub static JSON_HEADER: Lazy<Header> = Lazy::new(|| Header {
    field: HeaderField::from_str("Content-Type").unwrap(),
    value: create_ascii_string("application/json".to_string()),
});

//
// #[derive(Serialize, Hash, Eq, PartialEq)]
// // Struct copy from sysinfo to implement Serde traits
// pub(crate) struct CPUsageCalculationValues {
//     pub(crate) old_process_sys_cpu: u64,
//     pub(crate) old_process_user_cpu: u64,
//     pub(crate) old_system_sys_cpu: u64,
//     pub(crate) old_system_user_cpu: u64,
// }
//
// #[derive(Serialize, PartialEq)]
// pub struct Process {
//     pub inner: ProcessInner,
// }
//
// #[derive(Serialize, Hash, Eq, PartialEq, Debug)]
// pub struct HandleWrapper(pub usize);
//
// #[derive(Serialize, PartialEq)]
// // Struct copy from sysinfo to implement Serde traits
// pub struct ProcessInner {
//     pub(crate) name: String,
//     pub(crate) cmd: Vec<String>,
//     pub(crate) exe: Option<PathBuf>,
//     pub(crate) pid: Pid,
//     pub(crate) user_id: Option<Uid>,
//     pub(crate) environ: Vec<String>,
//     pub(crate) cwd: Option<PathBuf>,
//     pub(crate) root: Option<PathBuf>,
//     pub(crate) memory: u64,
//     pub(crate) virtual_memory: u64,
//     pub(crate) parent: Option<Pid>,
//     pub(crate) status: ProcessStatus,
//     pub(crate) handle: Option<Arc<HandleWrapper>>,
//     pub(crate) cpu_calc_values: CPUsageCalculationValues,
//     pub(crate) start_time: u64,
//     pub(crate) run_time: u64,
//     pub(crate) cpu_usage: f32,
//     pub(crate) updated: bool,
//     pub(crate) old_read_bytes: u64,
//     pub(crate) old_written_bytes: u64,
//     pub(crate) read_bytes: u64,
//     pub(crate) written_bytes: u64,
// }
//


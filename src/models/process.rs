use std::ffi::OsStr;
use std::path::Path;
use serde::Serialize;
use sysinfo::ProcessStatus as SysProcessStatus;

impl From<SysProcessStatus> for ProcessStatus {
    fn from(status: SysProcessStatus) -> Self {
        match status {
            SysProcessStatus::Idle => ProcessStatus::Idle,
            SysProcessStatus::Run => ProcessStatus::Run,
            SysProcessStatus::Sleep => ProcessStatus::Sleep,
            SysProcessStatus::Stop => ProcessStatus::Stop,
            SysProcessStatus::Zombie => ProcessStatus::Zombie,
            SysProcessStatus::Tracing => ProcessStatus::Tracing,
            SysProcessStatus::Dead => ProcessStatus::Dead,
            SysProcessStatus::Wakekill => ProcessStatus::Wakekill,
            SysProcessStatus::Waking => ProcessStatus::Waking,
            SysProcessStatus::Parked => ProcessStatus::Parked,
            SysProcessStatus::LockBlocked => ProcessStatus::LockBlocked,
            SysProcessStatus::UninterruptibleDiskSleep => ProcessStatus::UninterruptibleDiskSleep,
            SysProcessStatus::Unknown(code) => ProcessStatus::Unknown(code),
        }
    }
}

#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
// Struct copy from sysinfo to implement Serde traits
pub enum ProcessStatus {
    Idle,
    Run,
    Sleep,
    Stop,
    Zombie,
    Tracing,
    Dead,
    Wakekill,
    Waking,
    Parked,
    LockBlocked,
    UninterruptibleDiskSleep,
    Unknown(u32),
}

#[derive(Serialize, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct DiskUsage {
    /// Total number of written bytes.
    pub total_written_bytes: u64,
    /// Number of written bytes since the last refresh.
    pub written_bytes: u64,
    /// Total number of read bytes.
    pub total_read_bytes: u64,
    /// Number of read bytes since the last refresh.
    pub read_bytes: u64,
}

#[derive(Serialize)]
pub struct ProcessInfo<'a> {
    pub(crate) name: String,
    pub(crate) disk_usage: DiskUsage,
    pub(crate) cpu_usage: f32,
    pub(crate) mem_usage: u64,
    pub(crate) pid: usize,
    pub(crate) virt_mem_usage: u64,
    pub(crate) proc_path: Option<&'a Path>,
    pub(crate) parent_pid: Option<u32>,
    pub(crate) workdir: Option<&'a Path>,
    pub(crate) run_time: u64,
    pub(crate) start_time: u64,
    pub(crate) status: ProcessStatus,
}

#[derive(Serialize)]
// Struct copy from sysinfo to implement Serde traits
pub struct Pid(pub u32);
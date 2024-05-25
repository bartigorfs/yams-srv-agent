use std::ffi::OsString;
use std::path::{PathBuf};
use serde::Serialize;
use sysinfo::DiskKind as SysDiskKind;

impl From<SysDiskKind> for DiskKind {
    fn from(disk_kind: SysDiskKind) -> Self {
        match disk_kind {
            SysDiskKind::HDD => DiskKind::HDD,
            SysDiskKind::SSD => DiskKind::SSD,
            SysDiskKind::Unknown(isize) => DiskKind::Unknown(isize)
        }
    }
}

#[derive(Serialize)]
pub(crate) struct DiskInner {
    pub(crate) type_: DiskKind,
    pub(crate) name: OsString,
    pub(crate) file_system: OsString,
    pub(crate) mount_point: PathBuf,
    pub(crate) total_space: u64,
    pub(crate) available_space: u64,
    pub(crate) is_removable: bool,
}

#[derive(Serialize)]
pub enum DiskKind {
    HDD,
    SSD,
    Unknown(isize),
}
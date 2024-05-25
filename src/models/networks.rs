use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct NetworksInner {
    pub(crate) name: String,
    pub(crate) data: NetworkData,
}

#[derive(Serialize)]
pub struct NetworkData {
    pub(crate) inner: NetworkDataInner,
}

#[derive(Serialize)]
pub(crate) struct NetworkDataInner {
    pub(crate) total_received: u64,
    pub(crate) received: u64,
    pub(crate) packets_received: u64,
    pub(crate) errors_on_received: u64,
    pub(crate) packets_transmitted: u64,
    pub(crate) total_errors_on_transmitted: u64,
    pub(crate) total_transmitted: u64,
    pub(crate) transmitted: u64,
    pub(crate) errors_on_transmitted: u64,
    pub(crate) total_errors_on_received: u64,
    pub(crate) total_packets_received: u64,
    pub(crate) total_packets_transmitted: u64,
    // pub(crate) mac_addr: MacAddr,
}
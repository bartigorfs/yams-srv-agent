use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct GenericInfo {
    pub(crate) total: u64,
    pub(crate) used: u64,
    pub(crate) total_swap: u64,
    pub(crate) used_swap: u64,
    pub(crate) cpu_amount: usize
}
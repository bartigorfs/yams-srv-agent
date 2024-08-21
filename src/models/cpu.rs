use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct SingleCpu {
    pub(crate) name: String,
    pub(crate) load: f64,
}
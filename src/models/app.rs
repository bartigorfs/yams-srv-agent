use std::collections::HashSet;
use std::sync::Arc;

pub struct AppConfig {
    pub host: Vec<u16>,
    pub port: u16,
    pub trusted_origins: Arc<HashSet<String>>,
}
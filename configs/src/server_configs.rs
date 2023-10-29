use serde_derive::{Deserialize, Serialize};

use crate::ConfigTrait;

pub const SERVER_CONFIGS_NAME: &str = "server";

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct ServerConfigs {
    pub root_dir: String,
    pub address: String,
    pub port: String,
    pub secret_code: String,
    pub use_tls: bool,
    pub login_limit: u8,
    // pub managers_path: Vec<String>,
    pub events_dbs_dir: String,
    pub language_code: String,
    pub log_dir: String,
    pub log_level: String,
}

impl ConfigTrait for ServerConfigs {
    fn name() -> &'static str {
        return SERVER_CONFIGS_NAME;
    }
}

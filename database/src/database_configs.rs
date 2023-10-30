use configs::ConfigTrait;
use serde_derive::{Deserialize, Serialize};

pub const DATABASE_CONIFIGS_NAME: &str = "database";

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct DatabaseConfigs {
    pub name: String,
    pub address: String,
    pub port: u16,
}

impl ConfigTrait for DatabaseConfigs {
    fn name() -> &'static str {
        return DATABASE_CONIFIGS_NAME;
    }
}

impl Default for DatabaseConfigs {
    fn default() -> Self {
        DatabaseConfigs {
            name: "cashmere_db".to_string(),
            address: "127.0.0.1".to_string(),
            port: 27017,
        }
    }
}
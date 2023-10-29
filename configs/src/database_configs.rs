use serde_derive::{Deserialize, Serialize};

use crate::ConfigTrait;

const DATABASE_CONIFIGS_NAME: &str = "database";

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

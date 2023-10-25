use serde_derive::Deserialize;

#[derive(Deserialize, Clone)]
pub struct DatabaseConfigs {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub search_engine_index_root: String,
}

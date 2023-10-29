use serde_derive::{Deserialize, Serialize};

use crate::ConfigTrait;

const TLSCONFIGS_NAME: &str = "tls";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TlsConfigs {
    pub server_key_path: String,
    pub server_ca_path: String,
    pub client_ca_path: String,
}

impl ConfigTrait for TlsConfigs {
    fn name() -> &'static str {
        TLSCONFIGS_NAME
    }
}

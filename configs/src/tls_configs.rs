use std::sync::OnceLock;

use dependencies_sync::rust_i18n::{self, t};
use serde_derive::{Deserialize, Serialize};

use crate::{ConfigTrait, get_config};

const TLSCONFIGS_NAME: &str = "tls";

static TLSCONFIGS: OnceLock<TlsConfigs> = OnceLock::new();

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

    fn get() -> &'static Self {
        if let Some(configs) = TLSCONFIGS.get() {
            return configs;
        } else {
            let configs = get_config::<TlsConfigs>().expect(t!("取得配置失败").as_str());
            TLSCONFIGS.set(configs).expect("设置配置失败");
        }

        TLSCONFIGS.get().unwrap()
    }
}

impl Default for TlsConfigs {
    fn default() -> Self {
        TlsConfigs {
            server_key_path: "tls/server.key".to_string(),
            server_ca_path: "tls/server.crt".to_string(),
            client_ca_path: "tls/client.crt".to_string(),
        }
    }
}

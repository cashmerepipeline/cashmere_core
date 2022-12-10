use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct TlsConfigs {
    pub server_key_path: String,
    pub server_ca_path: String,
    pub client_ca_path: String,
}

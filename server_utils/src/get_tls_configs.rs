use std::fs;

use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::transport::{ServerTlsConfig, Identity, Certificate};

pub fn get_tls_configs(configs: &configs::Configs) -> ServerTlsConfig{
    let server_key_path: &String = &configs.tls.server_key_path;
    let server_ca_path: &String = &configs.tls.server_ca_path;
    let client_ca_path: &String = &configs.tls.client_ca_path;

    // tls文件读取
    let cert = if let Ok(r) = fs::read(server_ca_path) {
        r
    } else {
        panic!("{}", t!("读取服务证书文件失败"))
    };

    let key = if let Ok(r) = fs::read(server_key_path) {
        r
    } else {
        panic!("{}", t!("读取服务密钥文件失败"))
    };

    let server_identity = Identity::from_pem(cert, key);
    let client_ca_cert = if let Ok(r) = fs::read(client_ca_path) {
        r
    } else {
        panic!("{}", t!("读取客户端证书文件失败"))
    };
    let client_ca_cert = Certificate::from_pem(client_ca_cert);

    ServerTlsConfig::new()
        .identity(server_identity)
        .client_ca_root(client_ca_cert)
}

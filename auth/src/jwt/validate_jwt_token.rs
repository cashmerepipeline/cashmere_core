use configs::{ServerConfigs, ConfigTrait};
use jsonwebtoken::crypto::verify;
use jsonwebtoken::{Algorithm, DecodingKey};

pub fn validate_jwt_token(token: &String) -> bool {
    // let server_configs = configs::get_server_configs();
    let server_configs = ServerConfigs::get();
    let secret_code = server_configs.secret_code.as_bytes();

    let splits: Vec<&str> = token.split('.').collect();
    let signature = splits[2];
    let message = format!("{}.{}", splits[0], splits[1]);

    let result = verify(
        signature,
        &message,
        &DecodingKey::from_secret(secret_code),
        Algorithm::HS512,
    )
    .unwrap();

    result
}

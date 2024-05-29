use configs::{ServerConfigs, ConfigTrait};
use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use dependencies_sync::chrono::Utc;
use crate::jwt::Claims;

// 生成jwt token
pub async fn gen_jwt_token(
    phone: &str,
    name: &str,
    orgnizations: &[String],
    departments: &[String],
    roles: &[String],
) -> Option<String> {
    // let server_configs = configs::get_server_configs();
    let server_configs = ServerConfigs::get();
    let secret_code = server_configs.secret_code.as_bytes();

    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some(phone.to_string());
    let claims: Claims = Claims {
        aud: phone.to_string(),
        name: name.to_string(),
        org: orgnizations.to_vec(),
        dpt: departments.to_vec(),
        exp: (Utc::now().timestamp() + 60 * 60 * 24 * 7) as usize,
        iat: Utc::now().timestamp() as usize,
        iss: "grpc.cashmere.swb".to_string(),
        sub: "cashmere".to_string(),
        roles: roles.to_vec(),
    };

    let token = encode(&header, &claims, &EncodingKey::from_secret(secret_code)).unwrap();

    Some(token)
}

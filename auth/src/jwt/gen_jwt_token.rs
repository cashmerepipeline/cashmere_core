use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use chrono::Utc;
use crate::jwt::Claims;

// 生成jwt token
pub async fn gen_jwt_token(
    phone: &String,
    name: &String,
    orgnizations: &Vec<String>,
    departments: &Vec<String>,
    roles: &Vec<String>,
) -> Option<String> {
    let server_configs = configs::get_server_configs();
    let secret_code = server_configs.secret_code.as_bytes();

    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some(phone.clone());
    let claims: Claims = Claims {
        aud: phone.clone(),
        name: name.clone(),
        org: orgnizations.clone(),
        dpt: departments.clone(),
        exp: (Utc::now().timestamp() + 60 * 60 * 24 * 7) as usize,
        iat: Utc::now().timestamp() as usize,
        iss: "grpc.cashmere.swb".to_string(),
        sub: "cashmere".to_string(),
        roles: roles.clone(),
    };

    let token = encode(&header, &claims, &EncodingKey::from_secret(secret_code)).unwrap();

    Some(token)
}

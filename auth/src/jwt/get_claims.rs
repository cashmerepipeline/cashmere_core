use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use crate::jwt::Claims;

// claims utils
pub fn get_claims(token: &String) -> Option<Claims> {
    let configs = configs::get_configs();
    let secret_code = configs.server.secret_code.as_bytes();

    match jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_code),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(c) => Some(c.claims),
        Err(_err) => None,
    }
}

use crate::jwt;
use crate::jwt::get_claims;

// 比较token aud 和 phone
pub async fn validate_is_owner(id: &String, token: &String) -> bool {
    let c = match get_claims::get_claims(token) {
        Some(c) => c.aud,
        None => return false,
    };

    return id.as_bytes() == c.as_bytes();
}

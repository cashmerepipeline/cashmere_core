use crate::jwt::get_claims;

pub async fn validate_is_root(token: &String) -> bool {
    let r = match get_claims::get_claims(token) {
        Some(c) => c.roles,
        None => return false,
    };

    if r.contains(&"1000000".to_string()) || r.contains(&"1000000".to_string()) {
        return true;
    }

    false
}

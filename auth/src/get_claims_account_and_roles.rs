use crate::jwt::get_claims;

pub fn get_claims_account_and_roles(token: &str) -> Option<(String, Vec<String>)> {
    let results = match get_claims(token) {
        Some(c) => (c.aud, c.roles),
        None => return None,
    };

    Some(results)
}

use crate::jwt::get_claims;

pub fn get_claims_account_and_roles(token: &String) -> Option<(String, Vec<String>)> {
    let user = match get_claims(token) {
        Some(c) => (c.aud, c.roles),
        None => return None,
    };

    Some(user)
}

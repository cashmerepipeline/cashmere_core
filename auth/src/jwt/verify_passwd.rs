// 校验密码
pub async fn verify_passwd(passwd: &String, hash: &String) -> Option<bool> {
    let passwd = passwd.as_bytes();
    match argon2::verify_encoded(hash, passwd) {
        Ok(r) => Some(r),
        Err(_) => None,
    }
}

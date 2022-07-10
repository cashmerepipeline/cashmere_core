use chrono::Utc;
use argon2::Config;

// 加密密码
pub async fn hash_password(passwd: &String) -> Option<String> {
    let password = passwd.as_bytes();
    let now = Utc::now().to_rfc3339();
    let salt = now.as_bytes();

    match argon2::hash_encoded(password, salt, &Config::default()) {
        Ok(h) => Some(h),
        Err(_) => None,
    }
}

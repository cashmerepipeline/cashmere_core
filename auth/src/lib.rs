/*
Project: grpc_server
Creator: 闫刚
Create time: 2020-09-10 08:11
Introduction:
*/

pub mod check;
pub mod jwt;
use tonic::metadata::{MetadataMap};

pub fn get_auth_token(metadata: &MetadataMap) -> Option<String> {
    match metadata.get("authorization") {
        Some(token) => {
            let auth_string = token.to_str().unwrap().to_string();
            // 分开Bearer 和 token
            let split_strings: Vec<&str> = auth_string.split(' ').collect();
            let token_string = split_strings[1].to_string();

            Some(token_string)
        }
        None => None,
    }
}

pub fn get_claims_account_and_roles(token: &String) -> Option<(String, Vec<String>)> {
    let user = match jwt::get_claims(token) {
        Some(c) => (c.aud, c.roles),
        None => return None,
    };

    Some(user)
}

#[cfg(test)]
mod tests {
    use crate::jwt;

    #[test]
    fn it_works() {
        use futures;

        let root_passwd =
            futures::executor::block_on(jwt::hash_password(&"root".to_string())).unwrap();
        let user_passwd =
            futures::executor::block_on(jwt::hash_password(&"user".to_string())).unwrap();
        // println!("{}", root_passwd);
        // println!("{}", user_passwd);
        // assert_eq!("$argon2i$v=19$m=4096,t=3,p=1$MjAyMC0xMS0yNlQxNjoyODoxNS40NzE2MzcrMDA6MDA$JuVkdGb42sZRWEVer/mDWx9EpUmfLf7pwOp2Aw4zas4".as_bytes(), root_passwd.as_bytes());
        // assert_eq!("$argon2i$v=19$m=4096,t=3,p=1$MjAyMC0xMS0yNlQxNjoyODoxNS44MjYxNDErMDA6MDA$Mmu9/gnABeCb/d4QMCyoDcoV8KSoPNtPPHCBL/xMCuQ".as_bytes(), user_passwd.as_bytes());
    }
}

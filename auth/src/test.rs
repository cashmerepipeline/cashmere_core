#[cfg(test)]
mod tests {
    

    #[test]
    fn it_works() {
        use futures;
        use crate::jwt::hash_password;

        let _root_passwd =
            futures::executor::block_on(hash_password(&"root".to_string())).unwrap();
        let _user_passwd =
            futures::executor::block_on(hash_password(&"user".to_string())).unwrap();
        // println!("{}", root_passwd);
        // println!("{}", user_passwd);
        // assert_eq!("$argon2i$v=19$m=4096,t=3,p=1$MjAyMC0xMS0yNlQxNjoyODoxNS40NzE2MzcrMDA6MDA$JuVkdGb42sZRWEVer/mDWx9EpUmfLf7pwOp2Aw4zas4".as_bytes(), root_passwd.as_bytes());
        // assert_eq!("$argon2i$v=19$m=4096,t=3,p=1$MjAyMC0xMS0yNlQxNjoyODoxNS44MjYxNDErMDA6MDA$Mmu9/gnABeCb/d4QMCyoDcoV8KSoPNtPPHCBL/xMCuQ".as_bytes(), user_passwd.as_bytes());
    }
}

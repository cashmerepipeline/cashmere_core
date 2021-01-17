/*
Author: 闫刚 (yes7rose@sina.com)
database.rs (c) 2020
Desc: 同步数据库操作
Created:  2020-11-23T11:57:54.268Z
Modified: !date!
*/


use mongodb::sync::{Client, Database};
use mongodb::options::{ClientOptions, StreamAddress};

pub fn get_cashmere_db(
    address: &String,
    port: &u16,
    name: &String
) -> Database {

    let options = ClientOptions::builder()
        .hosts(vec![StreamAddress {
            hostname: address.clone(),
            port: Some(port.clone()),
        }])
        .build();

    let client = match Client::with_options(options) {
        Ok(r) => r,
        Err(_e) => panic!("连接到数据库失败"),
    };

    client.database(name)
}

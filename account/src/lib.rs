/*
Author: 闫刚 (yes7rose@sina.com)
mod.rs (c) 2020
Desc:  账号管理模块
Created:  2020-09-17T01:00:33.046Z
Modified: !date!
*/

pub mod group;
pub mod view;

use bson::{doc, Document, Bson};

use entity;
use cash_core::results::*;

use cash_core::ids::ACCOUNTS_MANAGE_ID;
use cash_core::field::ids::*;

#[derive(Debug)]
pub enum AccountStatus {
    Stopped(i32),
    Actived(i32),
}

#[derive(Debug)]
pub enum LoginStatus {
    In(u8),
    Out,
}

pub async fn get_account_by_id(id: &String) -> Result<Document, OperationResult> {
    let result = entity::get_entity_by_id(&ACCOUNTS_MANAGE_ID.to_string(), &id).await;

    result
}

// 从doc中取得password
pub fn get_account_passwd_hash(doc: &Document) -> Option<String> {
    let pw_value = doc.get_str(&ACCOUNTS_PASSWORD_FIELD_ID.to_string());

    match pw_value {
        Ok(s) => Some(s.to_string()),
        Err(_e) => None,
    }
}

// 从doc中取得groups
pub fn get_account_groups(doc: &Document) -> Option<Vec<String>> {
    let groups: Vec<String> = match doc.get_array(&GROUPS_FIELD_ID.to_string()) {
        Ok(g) => g.iter().map(|x| x.as_str().unwrap().to_string()).collect(),
        Err(_e) => return None,
    };

    Some(groups)
}

/// 取得账号状态
pub fn is_account_stopped(doc: &Document) -> bool {
    let result = match doc.get_bool(&"stopped") {
        Ok(g) => g,
        Err(_e) => return false,
    };

    result
}

// 从doc中取得登录时间戳
pub fn get_account_login_timestamps(doc: &Document) -> Option<Vec<i64>> {
    let timstamps: Vec<i64> = match doc.get_array(&ACCOUNTS_LOGIN_TIMESTAMPS_FIELD_ID.to_string()) {
        Ok(ss) => ss.iter().map(|x| x.as_i64().unwrap()).collect(),
        Err(_e) => vec![]
    };

    Some(timstamps)
}

// pub fn get_account_manage_view_rules(doc: &Document) -> Option<Vec<String>>{
//     let view_rules:Vec<String> = match doc.get_array("view_rules") {
//         Ok(ss) => ss.iter().map(|x| x.as_i64().unwrap()).collect(),
//         Err(_e) => vec![]
//     };
//     Some(view_rules)
// }


// 更新登录时间戳
pub async fn update_account_login_timestamps(
    account_id: &String,
    timestamps: &Vec<i64>,
    new_timestapm: i64,
) -> Result<OperationResult, OperationResult> {
    let configs = configs::get_configs();

    let mut timestamps = timestamps.clone();
    // 没有超过最大登录限制则加入
    if timestamps.is_empty() || timestamps.len() < configs.server.login_limit as usize {
        timestamps.push(new_timestapm);
    } else {
        // 更新最晚登录时间戳
        let min = timestamps.iter().min().unwrap();
        let min_index = timestamps.iter().position(|x| x == min).unwrap();
        timestamps[min_index] = new_timestapm;
    }

    let query_doc = doc! {
        "_id": account_id.clone()
    };
    let modify_doc = doc! {
        ACCOUNTS_LOGIN_TIMESTAMPS_FIELD_ID.to_string():timestamps
    };

    let result =
        entity::update_entity_field(&ACCOUNTS_MANAGE_ID.to_string(),
                                    query_doc,
                                    modify_doc,
                                    account_id).await;

    match result {
        Ok(_r) => Ok(operation_succeed("ok")),
        Err(e) => Err(add_call_name_to_chain(e, "update_account_login_timestamps".to_string())),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

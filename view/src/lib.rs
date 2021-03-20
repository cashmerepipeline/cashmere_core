pub mod view_rule;

use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

use cash_result::OperationResult;
use view_rule::{ReadRule, WriteRule};
pub enum ViewLevel {
    Manage,
    Entity,
    Schema,
}

#[derive(Debug, Serialize, Deserialize)]
struct ManageViewClaims {
    // 管理
    manage: String,
    // 用户名
    account_id: String,
    // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    exp: usize,
    // Optional. Issued at (as UTC timestamp)
    iat: usize,
    // Optional. Issuer
    iss: String,
    // Optional. Subject (whom token refers to)
    sub: String,
    // 权限
    // view_map: LinkedHashMap<String, String>,
}

/// 生成映射token
// pub async fn gen_manage_view_token(
//     manange: &String,
//     account_id: &String,
//     groups: &Vec<String>
// ) -> Result<String, OperationResuslt> {
//     // 1. 取得管理列表
//     let manages = manage::get_manages(account_id, groups).await;

//     // 3. 取得组合账号映像规则
//     let account_doc = match account::get_account_by_id(&account_id).await {
//         Ok(d) => d,
//         Err(e) => return Err(e)
//     };

//     let groups = match account::get_account_groups(&account_doc) {
//         Some(g) => g,
//         None => return Err(operation_result::operation_failed("gen_manage_view_token", "取得账户组失败"))
//     };
//     let account_manage_view_ruls = match account::get_account_manage_view_rules(&account_doc) {};
//     // 2. 取得组映像规则

//     // 4. 组合映像规则

//     // 3. 生成映像token并返回
// unimplemented!();
// }

/// 验证映射token
pub async fn validate_view_token() -> bool {
    unimplemented!()
}

/// 验证是否主人
pub fn validate_is_owner(entity_doc: &Document, account_id: &String) -> bool {
    unimplemented!()
}

/// 是否在组中
pub fn validate_is_in_group(entity_doc: &Document, group_ids: &Vec<String>) -> bool {
    unimplemented!()
}

/// 管理是否可写, 管理的字段定义添加删除
pub async fn can_manage_write(
    account: &String, 
    groups: &Vec<String>, 
    manage_id: &String
) -> bool {
    let view_rules_arc = view_rule::get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let mut result = false;
    // println!("查看管理是否可写 {}", manage_id);
    // TODO: 异常处理完善
    for group in groups {
        let rule = &view_rules
            .get(manage_id)
            .unwrap()
            .manage
            .get(group)
            .unwrap()
            .write_rule;
        result = result
            || rule == &WriteRule::Write
            || rule == &WriteRule::OwnerWrite
            || rule == &WriteRule::GroupWrite;
    }
    result
}

/// 集合是否可写，向集合添加或者删除实体
pub async fn can_collection_write(
    account: &String, 
    groups: &Vec<String>, 
    manage_id: &String
) -> bool {
    let view_rules_arc = view_rule::get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    for group in groups {
        let rule = &view_rules
            .get(manage_id)
            .unwrap()
            .collection
            .get(group)
            .unwrap()
            .write_rule;
        if rule == &WriteRule::Write
            || rule == &WriteRule::OwnerWrite
            || rule == &WriteRule::GroupWrite
        {
            return true;
        }
    }
    // 不在可写组中
    false
}

/// 实体的可写性，可否修改实体的字段
pub async fn can_entity_write(
    account: &String,
    groups: &Vec<String>,
    manage_id: &String,
    field: &String,
) -> bool {
    let view_rules_arc = view_rule::get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let mut result = false;
    // TODO: 异常处理
    for group in groups {
        // println!("查看实体是否可写 {} {} {} {:?}", manage_id, field, group, view_rules.get(manage_id).unwrap().schema);
        let rule = &view_rules
            .get(manage_id)
            .unwrap()
            .schema
            .get(field)
            .unwrap()
            .get(group)
            .unwrap()
            .write_rule;
        result = result
            || rule == &WriteRule::Write
            || rule == &WriteRule::OwnerWrite
            || rule == &WriteRule::GroupWrite;
    }
    result
}


/// 取得第一个可写组
pub async fn get_first_write_group(
    groups: &Vec<String>, 
    manage_id: &String
) -> Option<String> {
    let view_rules_arc = view_rule::get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    for group in groups {
        let rule = &view_rules
            .get(manage_id)
            .unwrap()
            .collection
            .get(group)
            .unwrap()
            .write_rule;

        if rule == &WriteRule::Write
            || rule == &WriteRule::OwnerWrite
            || rule == &WriteRule::GroupWrite
        {
            return Some(group.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

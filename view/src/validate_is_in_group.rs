use bson::Document;

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



/// 是否在组中
pub fn validate_is_in_group(_entity_doc: &Document, _group_ids: &Vec<String>) -> bool {
    unimplemented!()
}

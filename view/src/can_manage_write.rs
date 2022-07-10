use crate::view_rules_map::get_view_rules_map;
use crate::WriteRule;

/// 管理是否可写, 管理的字段定义添加删除
pub async fn can_manage_write(_account: &String, groups: &Vec<String>, manage_id: &String) -> bool {
    let view_rules_arc = get_view_rules_map().await;
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

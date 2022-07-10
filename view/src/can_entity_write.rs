use crate::view_rules_map::get_view_rules_map;
use crate::WriteRule;

/// 实体的可写性，可否修改实体的字段
pub async fn can_entity_write(
    _account: &String,
    groups: &Vec<String>,
    manage_id: &String,
    field: &String,
) -> bool {
    let view_rules_arc = get_view_rules_map().await;
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

use crate::view_rules_map::query_collection_view_rules;
use crate::FilterRule;
use dependencies_sync::log::{debug, error};
use dependencies_sync::rust_i18n::{self, t};

/// 检查实体是否可写
pub async fn can_entity_write(
    manage_id: &str,
    entity_id: &str,
    account_id: &str,
    role_group: &str,
) -> bool {
    let view_rules = if let Some(r) = query_collection_view_rules(manage_id, role_group).await {
        r
    } else {
        return false;
    };

    if cfg!(debug_assertions) {
        debug!(
            "{}: {}-{}-{:?}",
            t!("实体可写权限"),
            manage_id,
            role_group,
            view_rules
        );
    }

    if view_rules.write_filters.contains(&FilterRule::NoLimit) {
        return true;
    }

    if cfg!(debug_assertions) {
        debug!("{}: {}-{}", t!("获取实体"), manage_id, entity_id);
    }

    let entity = if let Ok(r) = entity::get_entity_by_id(manage_id, entity_id, &[], &[]).await {
        r
    } else {
        error!("{}: {}-{}", t!("获取实体失败"), manage_id, entity_id);
        return false;
    };

    // 组可写，判断实体的所属组
    if cfg!(debug_assertions) {
        debug!("{}: {}-{}", t!("测试是否组可写"), manage_id, entity_id);
    }
    if view_rules.write_filters.contains(&FilterRule::OnlyGroup) {
        return match entity::get_entity_groups(&entity) {
            Some(groups) => groups.contains(&role_group.to_string()),
            None => false,
        };
    }

    // 只主人可写
    if cfg!(debug_assertions) {
        debug!("{}: {}-{}", t!("测试是否主人可写"), manage_id, entity_id);
    }
    if view_rules.write_filters.contains(&FilterRule::OnlyOwner) {
        return match entity::get_entity_owner(&entity) {
            Some(owner) => {
                if cfg!(debug_assertions) {
                    debug!("{}: {}, {}", t!("判定是否主人"), owner, account_id,);
                }

                owner == account_id.to_string()
            }
            None => false,
        };
    }

    false
}

pub fn get_member_view_name(owner_manage_id: &str, owner_entity_id: &str, self_manage_id:&str) -> String {
    format!("{}_{}_{}_view", owner_manage_id, owner_entity_id, self_manage_id)
}
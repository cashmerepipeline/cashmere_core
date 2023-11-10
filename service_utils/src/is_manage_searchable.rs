use std::ops::Deref;

use majordomo::get_majordomo;
use managers::ManagerTrait;

async fn is_manage_searchable(manage_id: i32) -> bool {
    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();
    manager.is_searchable().await
}

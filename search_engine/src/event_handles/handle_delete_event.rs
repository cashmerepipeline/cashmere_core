use dependencies_sync::tantivy::Term;
use majordomo::get_majordomo;
use managers::manager_trait::ManagerTrait;

use crate::{get_manage_index_writer, get_manage_tantivy_index, get_manage_tantivy_schema};

pub fn handle_delete_event(manage_id: i32, object_id: &String) {
    println!("handle_delete_event: {}-{}", manage_id, object_id);
    
    // zh: 大部分管理的实体不删除，只有少数的几个管理实体支持删除操作
    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();
    if manager.is_entity_deleteable() == false {
        return;
    }

    let schema = get_manage_tantivy_schema(manage_id).unwrap();
    let writer_arc = get_manage_index_writer(manage_id);
    let writer = writer_arc.read();

    writer.delete_term(Term::from_field_text(
        schema.get_field("_id").unwrap(),
        object_id.as_str(),
    ));

    let index = get_manage_tantivy_index(manage_id);

    // if let Ok(mut writer) = index.writer::<TantivyDocument>(15_000_000) {
    //     if let Err(e) = writer.commit() {
    //         log::error!("{}: {}: {:?}", t!("提交删除失败"), manage_id, e);
    //     };

    //     index.reader().unwrap().reload();
    // } else {
    //     log::error!("{}: {}", t!("获取writer失败"), manage_id);
    // }
}

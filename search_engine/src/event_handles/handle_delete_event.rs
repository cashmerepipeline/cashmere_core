use cash_result::operation_failed;
use dependencies_sync::log;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tantivy::{IndexWriter, TantivyDocument, Term};

use crate::{get_manage_tantivy_index, get_manage_tantivy_schema};

pub fn handle_delete_event(manage_id: i32, entity_id: &String) {
    println!("handle_delete_event: {}-{}", manage_id, entity_id);

    let index = get_manage_tantivy_index(manage_id);
    let schema = get_manage_tantivy_schema(manage_id).unwrap();

    if let Ok(mut writer) = index.writer::<TantivyDocument>(15_000_000) {
        writer.delete_term(Term::from_field_text(
            schema.get_field("_id").unwrap(),
            entity_id.as_str(),
        ));

        if let Err(e) = writer.commit() {
            log::error!("{}: {}: {:?}", t!("提交删除失败"), manage_id, e);
        };

        index.reader().unwrap().reload();
    } else {
        log::error!("{}: {}", t!("获取writer失败"), manage_id);
    }
}

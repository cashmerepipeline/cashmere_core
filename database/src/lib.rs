/*
使用单例模式创建数据库 client
所有操作只使用一个client, 需要进一步测试
*/

use dependencies_sync::rust_i18n::{self};
rust_i18n::i18n!("locales");

pub use get_mongodb_client::*;
pub use get_cashmere_database::*;
pub use collection_exists::*;
pub use get_collection_by_id::*;
pub use get_manages_collection::*;
pub use get_ids_collection::*;
pub use init_ids_count_field::*;

mod get_mongodb_client;
mod get_cashmere_database;
mod collection_exists;
mod get_collection_by_id;
mod get_manages_collection;
mod get_ids_collection;
mod init_ids_count_field;

#[cfg(test)]
mod tests {
    use crate::get_cashmere_database;
    use dependencies_sync::mongodb::bson::doc;
    use tokio_test::assert_ok;

    #[test]
    fn test_database() {
        let db = tokio_test::block_on(get_cashmere_database::get_cashmere_database());
        tokio_test::block_on(db.create_collection("test", None)).expect("创建测试集合失败");
        let collection = db.collection("test");
        let doc = doc! {
            "name": "Test"
        };
        let result = tokio_test::block_on(collection.insert_one(doc.clone(), None));
        assert_ok!(result);
        let f_doc = tokio_test::block_on(collection.find_one(doc.clone(), None))
            .unwrap()
            .unwrap();
        assert_eq!(doc.get_str("name").unwrap(), f_doc.get_str("name").unwrap());
    }
}

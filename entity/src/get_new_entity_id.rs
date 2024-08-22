use std::collections::BTreeMap;

use dependencies_sync::chrono::Utc;
// use dependencies_sync::tokio::stream::StreamExt;
use dependencies_sync::futures::stream::StreamExt;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::log::{self, debug};
use dependencies_sync::mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};
use dependencies_sync::mongodb::{bson, bson::doc, bson::Bson, bson::Document, Collection};
use dependencies_sync::rust_i18n::{self, t};
use manage_define::hard_coded_field_names::ID_ENUNM_FIELD_NAME;
use serde::Deserialize;

use cash_result::*;
use database::{get_database, init_ids_count_field};
use manage_define::general_field_ids::*;

use crate::utils::get_timestamp_update_doc;

/// 取得新连续id
/// 数据库初始化后新建实体需要保证编号次序
pub async fn get_new_entity_id(manage_id: &str, account_id: &str) -> Option<i64> {
    let ids_collection = database::get_ids_collection().await;
    if let Err(e) = init_ids_count_field(manage_id).await{
        log::error!("{}: {}", t!("初始化编号字段失败"), e.details());
        panic!();
    };
    
    let update_doc = doc! {
     "$inc": {ID_ENUNM_FIELD_NAME:1i64},
     "$set": { MODIFIER_FIELD_ID.to_string(): account_id},
     "$currentDate": {
         MODIFY_TIMESTAMP_FIELD_ID.to_string(): { "$type": "timestamp" }
      },
    };

    let result = ids_collection
        .find_one_and_update(
            doc! {
                OID_FIELD_ID: manage_id
            },
            update_doc,
        )
        .upsert(true)
        .await;

    match result {
        Ok(r) => {
            if let Some(r) = r {
                if cfg!(debug_assertions) {
                    debug!("{}: {:?}", t!("新实体"), r);
                }

                if let Ok(r) = r.get_i64(ID_ENUNM_FIELD_NAME) {
                    Some(r)
                } else {
                    log::error!("{}: {}", t!("获取新实体编号失败"), manage_id);
                    None
                }
            } else {
                // 不存在
                log::error!("{}: {}", t!("新实体编号数据错误"), manage_id);
                None
            }
        }
        Err(_e) => {
            log::error!("{}: {}", t!("取得新实体编号失败"), manage_id);
            None
        }
    }
}

use cash_result::{add_call_name_to_chain, OperationResult};
use dependencies_sync::rust_i18n::{self, t};
use view::get_manage_schema_view_mask;

use manage_define::general_field_ids::REMOVED_FIELD_ID;



use dependencies_sync::log::{error};



use dependencies_sync::bson::doc;



use majordomo::get_majordomo;
use managers::ManagerTrait;

use dependencies_sync::bson::Document;

pub(crate) async fn get_manage_entities_page(
    _account_id: &str,
    role_group: &str,
    manage_id: &str,
    match_doc: &Document,
    sort_doc: &Option<Document>,
    page_index: &u32,
    no_present_fields: &[String],
) -> Result<Vec<Document>, OperationResult> {
    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    let mut matches = doc! {};
    matches.insert(REMOVED_FIELD_ID.to_string(), false);
    for (k, v) in match_doc {
        matches.insert(k.clone(), v.clone());
    }

    let fields = manager.get_manage_schema().await;
    let mut unsets =
        get_manage_schema_view_mask(manage_id, &fields, &role_group.to_string())
            .await
            .iter()
            .filter(|(_k, v)| !(**v))
            .map(|(k, _v)| k.clone())
            .collect::<Vec<String>>();
    
    no_present_fields.iter().for_each(|k| unsets.push(k.clone()));

    let result = manager
        .get_entities_by_page(*page_index, &Some(matches), sort_doc, &unsets)
        .await;

    match result {
        Ok(r) => Ok(r),
        Err(e) => {
            error!("{}: {}", t!("获取集合数据失败"), e.details());
            Err(add_call_name_to_chain(
                e,
                "get_manage_entities_page".to_string(),
            ))
        }
    }
}

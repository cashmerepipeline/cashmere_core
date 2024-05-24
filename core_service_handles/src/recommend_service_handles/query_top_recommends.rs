use cash_result::{operation_failed, OperationResult};
use dependencies_sync::{bson::{doc, Document}, futures::StreamExt, log::error, rust_i18n::{self, t}};
use manage_define::{field_ids::{RECOMMENDS_ENTITY_ID_FIELD_ID, RECOMMENDS_MANAGE_ID_FIELD_ID}, manage_ids::RECOMMENDS_MANAGE_ID};

/// 取得最高推荐测实体列表，从高到低排序, 最多返回1000个
pub async fn query_top_recommends(manage_id: &str, count: &i32) -> Result<Vec<Document>, OperationResult> {
  let collection = match database::get_collection_by_id(RECOMMENDS_MANAGE_ID).await{
    Some(c) => c,
    None => return Err(operation_failed("query_top_recommends", t!("取得数据库集合失败"))),
  };

  let match_doc = doc! {
    "$match": {
        RECOMMENDS_MANAGE_ID_FIELD_ID.to_string(): manage_id,
    }
  };
  
  let query_doc = doc! {
    "$group": {
      "_id": format!("${}", RECOMMENDS_ENTITY_ID_FIELD_ID),
      "count": {"$sum": 1},
    }
  };
  
  let sort_doc = doc! {
    "$sort": {
      "count": -1,
    }
  };
  
  let result = collection.aggregate([match_doc, query_doc, sort_doc], None).await;
  match result {
    Ok(mut results) => {
      let mut result_list = Vec::new();
      while let Some(result) = results.next().await {
        if let Err(err) = &result {
          error!("{}: {}", t!("查询数据库失败"), err);
          continue;
        }

        result_list.push(result.unwrap());
        
        if result_list.len() >= *count as usize || result_list.len() >= 1000 { // 最多返回1000个
          break;
        }
      }

      Ok(result_list)
  }
    Err(err) =>  Err(operation_failed("query_top_recommends", format!("{}: {}", t!("查询数据库失败"), err))) 
  }
}
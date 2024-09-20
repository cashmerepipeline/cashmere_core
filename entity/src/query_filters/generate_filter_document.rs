use cash_result::{operation_failed, OperationResult};
use dependencies_sync::{
    bson::{doc, from_slice, Document},
    log::debug,
    rust_i18n::{self, t},
};
use manage_define::cashmere::{QueryFilter, QueryFilterType};

pub fn generate_filter_document(filters: &Vec<QueryFilter>) -> Result<Document, OperationResult> {
    let mut result = Document::new();
    for filter in filters {
        let filter_type = if let Ok(t) = QueryFilterType::try_from(filter.filter_type) {
            t
        } else {
            return Err(operation_failed(
                "generate_filter_document",
                t!("不是有效的查询过滤类型"),
            ));
        };

        // 取得和检查值
        let value = if let Ok(d) = from_slice::<Document>(&filter.value) {
            if let Some(r) = d.get("value") {
                r.clone()
            } else {
                return Err(operation_failed(
                    "generate_filter_document",
                    t!("不是有效的查询值"),
                ));
            }
        } else {
            if cfg!(debug_assertions) {
                debug!("{}: {:?}", t!("不是有效的查询文档"), filter);
            }
            return Err(operation_failed(
                "generate_filter_document",
                t!("不是有效的查询文档"),
            ));
        };

        // 根据类型，生成mongodb查询文档
        match filter_type {
            QueryFilterType::Match => {
                result.insert(filter.field_id.clone(), value);
            }
            QueryFilterType::GreaterThan => {
                result.insert(filter.field_id.clone(), doc! { "$gt": value });
            }
            QueryFilterType::GreaterThanOrEqual => {
                result.insert(filter.field_id.clone(), doc! { "$gte": value });
            }
            QueryFilterType::LessThan => {
                result.insert(filter.field_id.clone(), doc! { "$lt": value });
            }
            QueryFilterType::LessThanOrEqual => {
                result.insert(filter.field_id.clone(), doc! { "$lte": value });
            }
            QueryFilterType::In => {
                result.insert(filter.field_id.clone(), doc! { "$in": value });
            }
            QueryFilterType::NotIn => {
                result.insert(filter.field_id.clone(), doc! { "$nin": value });
            }
            QueryFilterType::All => {
                result.insert(filter.field_id.clone(), doc! { "$all": value });
            }
            QueryFilterType::Exists => {
                result.insert(filter.field_id.clone(), doc! { "$exists": value });
            }
            QueryFilterType::Equal => {
                result.insert(filter.field_id.clone(), doc! { "$eq": value });
            }
            QueryFilterType::NotEqual => {
                result.insert(filter.field_id.clone(), doc! { "$ne": value });
            }
        }
    }

    Ok(result)
}

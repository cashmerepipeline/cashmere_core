use std::ops::Deref;

use cash_result::{operation_failed, OperationResult};

use dependencies_sync::tantivy::{collector::TopDocs, query::QueryParser, schema::*};
use dependencies_sync::{
    log,
    rust_i18n::{self, t},
};
use manage_define::general_field_ids::{DESCRIPTION_FIELD_ID, NAME_MAP_FIELD_ID};

use crate::tantivy_searcher::get_tantivy_searcher;
use crate::{get_tantivy_schema, get_tantivy_index};


pub async fn search(manage_id: i32, search_str: &str) -> Result<Vec<String>, OperationResult> {
    
    
    log::debug!("{}: {}-{:?}", t!("搜索开始"), manage_id, search_str);

    // let schema = if let Some(s) = get_manage_tantivy_schema(manage_id) {
    //     s
    // } else {
    //     return Err(operation_failed("search", t!("索引不存在")));
    // };
    let schema = get_tantivy_schema();

    // log::debug!("{}: {}-{:?}", t!("取得索引模式"), manage_id, schema);
    
    let mid_f = schema.get_field("mid").unwrap();
    let name_map_f = schema
        .get_field(NAME_MAP_FIELD_ID.to_string().as_ref())
        .unwrap();
    let description_f = schema
        .get_field(DESCRIPTION_FIELD_ID.to_string().as_ref())
        .unwrap();

    let index_arc = get_tantivy_index();
    let index = index_arc.read();

    let query_parser = QueryParser::for_index(index.deref(), vec![mid_f, name_map_f, description_f]);
    let query = query_parser
        .parse_query(search_str.to_string().as_str())
        .map_err(|err| {
            operation_failed(
                "search",
                format!(
                    "{}: {}-{:?}, {:?}",
                    t!("建立查询失败"),
                    manage_id,
                    search_str,
                    err
                ),
            )
        })?;
    log::debug!("{}: {}-{:?}", t!("建立查询成功"), manage_id, query);

    let searcher = get_tantivy_searcher().unwrap();
    let top_docs = if let Ok(top_docs) = searcher.search(&query, &TopDocs::with_limit(100)) {
        log::debug!("{}: {}-{:?}", t!("搜索成功"), manage_id, top_docs);
        top_docs
    } else {
        log::error!("{}: {}-{:?}", t!("搜索失败"), manage_id, search_str);
        return Err(operation_failed(
            "search",
            format!("{}: {}-{:?}", t!("搜索失败"), manage_id, search_str),
        ));
    };

    let mut result = vec![];
    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address).unwrap();

        let j_doc = retrieved_doc.to_json(&schema);
        result.push(j_doc);
    }

    log::debug!("{}: {}-{:?}", t!("搜索结束"), manage_id, result);

    Ok(result)
}

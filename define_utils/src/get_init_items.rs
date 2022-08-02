use bson::Document;

/// 初始记录
pub fn get_init_items(toml_map: &toml::map::Map<String, toml::Value>) -> Option<Vec<Document>> {
    let result = toml_map.get("items").unwrap().as_table();
    match result {
        Some(r) => {
            let mut items: Vec<Document> = vec![];
            for (_key, value) in r {
                // item is table
                let v_table = value.as_table().unwrap();
                if v_table.is_empty() {
                    continue;
                }
                // 构造Document
                let doc = bson::ser::to_document(v_table).unwrap();
                println!("取得初始化实体 {}", doc);
                items.push(doc);
            }
            Some(items)
        }
        None => None,
    }
}

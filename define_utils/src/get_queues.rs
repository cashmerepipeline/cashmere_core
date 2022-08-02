/// 管理队列
pub fn get_queues(toml_map: &toml::map::Map<String, toml::Value>) -> Option<Vec<String>> {
    let result = toml_map.get("queues").unwrap().as_table();
    match result {
        Some(r) => {
            let mut queues: Vec<String> = vec![];
            for (_key, value) in r {
                println!("{:?}", value);
                if let Some(r) = value.as_str() {
                    queues.push(r.to_string());
                }
            }
            Some(queues)
        }
        None => None,
    }
}

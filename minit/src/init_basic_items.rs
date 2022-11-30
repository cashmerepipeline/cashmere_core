use toml::map::Map;
use toml::Value;
use define_utils as utils;

pub async fn init_basic_items(
    tomls: &Vec<Map<String, Value>>,
    root_id: &String,
    root_group_id: &String,
) {
    for map in tomls {
        let manage_id = utils::get_id(map).unwrap();
        // let collection = db.collection(&id.to_string());
        if let Some(items) = utils::get_init_items(map) {
            for mut item in items {
                if let Ok(_r) =
                    entity::insert_entity(&manage_id.to_string(), &mut item, root_id, root_group_id)
                        .await
                {
                    continue;
                } else {
                    println!("插入记录失败, {}", item);
                }
            }
        }
    }
}

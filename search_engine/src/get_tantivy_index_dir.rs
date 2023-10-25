use configs::get_configs;
use dependencies_sync::tantivy::directory::MmapDirectory;

pub fn get_tantivy_index_dir(manage_id: i32) -> String {
    let root_dir = &get_configs().database.search_engine_index_root;
    let index_dir = format!("{}/{}", root_dir, manage_id);

    return index_dir;
}

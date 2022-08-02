// 从文件列表创建toml表
pub fn get_tomls_from_pathes(
    toml_pathes: &Vec<String>,
) -> Option<Vec<toml::map::Map<String, toml::Value>>> {
    // 读入所有文件并构造toml映射
    let mut tomls: Vec<toml::map::Map<String, toml::Value>> = vec![];
    for toml_path in toml_pathes {
        // let mut toml_file = std::fs::File::open(toml_path).expect("初始化数据库文件不存在");
        //
        // let mut toml_string = "".to_string();
        // toml_file
        //     .read_to_string(&mut toml_string)
        //     .expect("管理定义文件读取错误");
        //
        // let toml_map: toml::map::Map<String, toml::Value> =
        //     toml::from_str(&toml_string).expect("管理定义文件定义错误");
        if let Some(toml_map) = crate::get_toml_map::get_toml_map(toml_path) {
            tomls.push(toml_map);
        }
    }

    Some(tomls)
}

use std::path::Path;

use dependencies_sync::log;

// 取得目录下的toml文件
pub fn get_toml_files_of_dir(toml_dir: &String) -> Option<Vec<String>> {
    let mut toml_pathes: Vec<String> = vec![];
    let toml_dir_path = Path::new(toml_dir);
    if toml_dir_path.exists() && toml_dir_path.is_dir() {
        for entry in toml_dir_path
            .read_dir()
            .unwrap_or_else(|_| panic!("{} 目录不存在或者不是目录", toml_dir))
        {
            let path_buf = &entry.unwrap().path();
            if path_buf.is_dir() {
                continue;
            }
            // 添加toml文件路径
            let path_string = path_buf.to_str().unwrap().to_string();
            if path_string.ends_with(".toml") {
                log::info!("取得管理定义文件 {}", path_string);
                toml_pathes.push(path_string);
            }
        }
    }

    Some(toml_pathes)
}

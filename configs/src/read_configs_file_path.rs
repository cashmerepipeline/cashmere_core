use dependencies_sync::rust_i18n::{self, t};

// 读取参数-c 或 --configs 指定的配置文件路径
pub fn read_configs_file_path() -> String {
    let mut configs_path = String::new();
    let mut args = std::env::args();
    args.next();
    while let Some(arg) = args.next() {
        if arg == "-c" || arg == "--configs" {
            configs_path = args.next().unwrap_or_default();
            break;
        }
    }

    // 如果没有指定配置文件路径, 则使用默认路径
    if configs_path.is_empty() {
        configs_path = String::from("./configs.toml");
    }
    
    // 如果配置文件不存在，则停止程序
    if !std::path::Path::new(&configs_path).exists() {
        println!("{}: {}", t!("配置文件不存在"), configs_path);
        std::process::exit(1);
    }
    
    configs_path
}
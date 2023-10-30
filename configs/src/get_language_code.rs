use crate::{get_config, SERVER_CONFIGS_NAME, ServerConfigs};

/// 取得服务器语言设置
pub fn get_language_code() -> String {
    let configs = get_config::<ServerConfigs>().unwrap();
    configs.language_code.clone()
}

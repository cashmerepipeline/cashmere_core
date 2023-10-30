use configs::get_config;
use search_engine::search_engine_configs::{SEARCH_ENGINE_CONFIG_NAME, SearchEngineConfigs};
use search_engine::get_search_engine_configs;

#[test]
fn get_search_engine_configs_test() {
  let configs = get_config::<SearchEngineConfigs>(&SEARCH_ENGINE_CONFIG_NAME.to_string());
  assert_eq!(configs.is_some(), true);
}
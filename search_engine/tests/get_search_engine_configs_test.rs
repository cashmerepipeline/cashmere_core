use configs::get_config;
use search_engine::SearchEngineConfigs;

#[test]
fn get_search_engine_configs_test() {
    let configs = get_config::<SearchEngineConfigs>();
    assert!(configs.is_some());
}

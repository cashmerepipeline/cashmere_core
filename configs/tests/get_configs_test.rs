use configs::{get_config, get_configs, init_configs_path, ServerConfigs, SERVER_CONFIGS_NAME};

#[test]
fn get_configs_test() {
    init_configs_path(String::from("configs_template.toml")).unwrap();
    let configs = get_configs();
    // let configs_map = get_configs_map();
    // let configs_map = configs_map.read();

    assert_eq!(configs.database.address, "localhost");
    assert_eq!(configs.database.port, 27017);
    assert_eq!(configs.server.address, "127.0.0.1");
    assert_eq!(configs.server.port, "8800");
    assert!(!configs.server.use_tls);
    assert_eq!(configs.server.login_limit, 2);
}

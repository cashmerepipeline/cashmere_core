use configs::{get_config, get_configs, init_configs_path, ServerConfigs, SERVER_CONFIGS_NAME};

#[test]
fn get_configs_from_map_test() {
    init_configs_path(String::from("configs_template.toml")).unwrap();
    let _configs = get_configs();
    // let configs_map = get_configs_map();
    // let configs_map = configs_map.read();

    let server_configs: Option<ServerConfigs> =
        get_config::<ServerConfigs>(&SERVER_CONFIGS_NAME.to_string());
    assert_eq!(server_configs.is_some(), true);

    assert_eq!(server_configs.as_ref().unwrap().address, "127.0.0.1");
    assert_eq!(server_configs.as_ref().unwrap().port, "8800");
    assert!(!server_configs.as_ref().unwrap().use_tls);
    assert_eq!(server_configs.as_ref().unwrap().login_limit, 2);
}

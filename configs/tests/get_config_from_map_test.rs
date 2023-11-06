use configs::{get_config, init_configs_file_path, init_configs_map, ServerConfigs};

#[test]
fn get_configs_from_map_test() {
    init_configs_file_path(String::from("configs_template.toml")).unwrap();
    let _ = init_configs_map();

    let server_configs: Option<ServerConfigs> = get_config::<ServerConfigs>();
    assert!(server_configs.is_some());

    assert_eq!(server_configs.as_ref().unwrap().address, "127.0.0.1");
    assert_eq!(server_configs.as_ref().unwrap().port, "8800");
    assert!(!server_configs.as_ref().unwrap().use_tls);
    assert_eq!(server_configs.as_ref().unwrap().login_limit, 2);
}

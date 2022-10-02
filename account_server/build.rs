fn main() {
    tonic_build::configure()
        .out_dir("src")
        .extern_path(".cashmere", "::manage_define::cashmere")
        .build_server(true)
        .build_client(false)
        .type_attribute("Name", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["protocols/account_service.proto"],
            &["protocols", "../protocols"],
        ).unwrap();
}

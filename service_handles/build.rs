fn main() {
    tonic_build::configure()
        .out_dir("src")
        .build_client(false)
        // .build_server(false)
        .type_attribute("Name", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Color", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "Position",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile(&["../protocols/cashmere.proto"], &["../protocols"])
        .unwrap();
}

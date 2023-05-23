use dependencies_sync::tonic_build;

fn main() {
    tonic_build::configure()
        .out_dir("../manage_define/src")
        .build_client(false)
        // .build_server(false)
        .type_attribute("Name", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Color", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Gender", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "DataServerConfigs",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "StageInfo",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute("Version", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "DataType",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "FileInfo",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "Position",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "SchemaField",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile(&["../protocols/cashmere.proto"], &["../protocols"])
        .unwrap();

    manage_define::utils::generate_manage_defines(
        &vec!["../manage_define/defines"],
        "../manage_define/src",
        Some("../dart_packages/general_id_codes/lib/src"),
    );
}

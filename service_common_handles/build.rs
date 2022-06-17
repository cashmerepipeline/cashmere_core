use manage_define;

fn main() {
    tonic_build::configure()
        .out_dir("src")
        .build_client(false)
        // .build_server(false)
        .type_attribute("Name", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Color", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("DataType", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("FileInfo", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "Position",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile(&["../protocols/cashmere.proto"], &["../protocols"])
        .unwrap();

    manage_define::generate_manage_defines(
        &vec!["../manage_define/defines/core"],
        "../manage_define/src",
        Some("../dart_packages/general_id_codes/lib/src"),
    );
}

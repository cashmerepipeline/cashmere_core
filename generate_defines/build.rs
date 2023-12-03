use dependencies_sync::tonic_build;

fn main() {
    tonic_build::configure()
        .out_dir("../manage_define/src")
        .build_client(false)
        .build_server(false)
        .type_attribute("Name", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Color", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Gender", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Position", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("PhoneAreaCode", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("FieldDataType", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Range", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Calendar", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("CalendarType", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["../protocols/cashmere.proto"], &["../protocols"])
        .unwrap();

    define_utils::generate_manage_defines(
        &vec!["../manage_define/defines"],
        "../manage_define/src",
        Some("../dart_packages/cashmere_core/lib/ids"),
    );
}

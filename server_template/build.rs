use manage_define;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/services")
        .extern_path(".cashmere", "::manage_define::cashmere")
        .build_client(false)
        .build_server(true)
        .compile_well_known_types(true)
        .compile(
            &["protocols/knitter.proto"],
            &["protocols", "../cashmere_core/protocols"],
        )?;

    define_utils::generate_manage_defines(
        &vec!["manage_defines"],
        "src/ids_codes",
        Some("dart_packges/knitter_id_codes/lib/src"),
    );

    Ok(())
}

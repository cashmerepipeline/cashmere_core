fn main() {
    tonic_build::configure()
        .out_dir("src")
        .build_client(false)
        .build_server(false)
        .compile(
            &["../protocols/cashmere.proto"],
            &["../protocols"],
        ).unwrap();
    tonic_build::configure()
        .out_dir("src")
        .build_client(false)
        .compile(
            &["../protocols/account.proto"],
            &["../protocols"],
        ).unwrap();
}

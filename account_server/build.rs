fn main() {
    tonic_build::configure()
        .out_dir("src")
        .compile(
            &["protocols/account.proto"],
            &["protocols"],
        ).unwrap();
}

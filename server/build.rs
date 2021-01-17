fn main() {
    tonic_build::configure()
    .out_dir("../server/src")
    .compile(
        &["../protocols/cashmere.proto", "../protocols/account.proto"],
        &["../protocols"]
    ).unwrap();
}

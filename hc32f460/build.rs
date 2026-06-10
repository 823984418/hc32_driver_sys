fn main() {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", manifest_dir.display());
    println!("cargo:rustc-link-lib=static=hc32_driver");
}

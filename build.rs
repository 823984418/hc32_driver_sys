fn main() {
    println!("cargo:rustc-env=HOST={}", std::env::var("HOST").unwrap());
}

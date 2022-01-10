use std::env;

fn main() {
    println!(
        "cargo:rustc-link-search=native={}", env!("CARGO_MANIFEST_DIR")
    );
}

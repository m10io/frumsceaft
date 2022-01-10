use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let memory_x = gpp::process_str(include_str!("memory.ld"), &mut gpp::Context::new()).unwrap();
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(memory_x.as_bytes())
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=memory.ld");

    println!("cargo:rustc-link-arg=--cmse-implib");
    println!(
        "cargo:rustc-link-arg=--out-implib={}",
        out.join("libnsclib.a").display()
    );
}

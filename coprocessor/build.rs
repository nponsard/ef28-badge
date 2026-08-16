use std::{env, fs::copy, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    copy("link.x", out.join("link.x")).unwrap();
    println!("cargo:rerun-if-changed=link.x");
    println!("cargo:rustc-link-search={}", out.display());
}

use std::path::PathBuf;

use anyhow::Result;

fn main() -> Result<()> {
    let path = PathBuf::from("src");
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path])
        .extra_clang_args(&["-I/usr/include/mozjs-128", "-std=c++20", "-DDEBUG=1"])
        .build()?;

    b.flag_if_supported("-std=c++20")
        .include("/usr/include/mozjs-128")
        .flag("-DDEBUG=1")
        .compile("spidermonkey-sys");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/spidermonkey.hpp");

    pkg_config::probe_library("mozjs-128")?;

    Ok(())
}

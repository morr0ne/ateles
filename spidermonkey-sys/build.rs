use anyhow::Result;

fn main() -> Result<()> {
    autocxx_build::Builder::new("src/lib.rs", &["src"])
        .extra_clang_args(&["-I/usr/include/mozjs-128", "-std=c++20"])
        .build()?
        .flag("-std=c++20")
        .include("/usr/include/mozjs-128")
        .compile("spidermonkey-sys");

    pkg_config::probe_library("mozjs-128")?;

    Ok(())
}

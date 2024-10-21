use anyhow::Result;

fn main() -> Result<()> {
    autocxx_build::Builder::new("src/lib.rs", &["src"])
        .extra_clang_args(&["-I/usr/include/mozjs-128", "-std=c++20", "-w"])
        .build()?
        .flag("-std=c++20")
        .flag("-w")
        .include("/usr/include/mozjs-128")
        .compile("spidermonkey-sys");

    pkg_config::Config::new().statik(true).probe("mozjs-128")?;

    Ok(())
}

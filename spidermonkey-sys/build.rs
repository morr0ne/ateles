use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    process::Command,
    thread::available_parallelism,
};

use anyhow::{bail, Context, Result};
use bindgen::{CodegenConfig, EnumVariation};
use tar::Archive;
use xz2::read::XzDecoder;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/spidermonkey.hpp");
    println!("cargo:rerun-if-changed=src/spidermonkey.cpp");

    let monkey_path = build_spidermonkey()?;

    generate_bindings(&monkey_path)?;

    cxx_build::bridges([
        "src/bindings/js.rs",
        "src/bindings/JS.rs",
        "src/bindings/root.rs",
    ])
    .file("src/spidermonkey.hpp")
    .file("src/spidermonkey.cpp")
    .cpp(true)
    .std("c++20")
    .flag("-w")
    .flag("-fPIC")
    .flag("-fno-rtti")
    .flag("-fno-exceptions")
    .flag("-DDEBUG=1")
    .include(monkey_path.join("dist/include"))
    .compile("spidermonkey-sys");

    println!(
        "cargo:rustc-link-search=native={}",
        monkey_path.join("js/src/build").display()
    );
    println!("cargo:rustc-link-lib=static=js_static");

    Ok(())
}

const SOURCE_URL: &str = "https://ftp.mozilla.org/pub/firefox/releases/128.3.1esr/source/firefox-128.3.1esr.source.tar.xz";
const FIREFOX_VERSION: &str = "128.3.1";
const CHECKSUM: &str = "3fa5ead3fb640dbf3253ba3d3d59550d99d1646a39144b9881a9f8897f18f650";

fn build_spidermonkey() -> Result<PathBuf> {
    let out_dir = build_dir();

    let build_path = out_dir.join("monkey-build");

    if out_dir.join(".monkey-ok").exists() {
        return Ok(build_path);
    }

    let download_path = out_dir.join("firefox.tar.xz");

    let already_downloaded = download_path.exists()
        && blake3::hash(&fs::read(&download_path)?)
            .to_hex()
            .to_string()
            == CHECKSUM;

    if !already_downloaded {
        if !Command::new("curl")
            .arg("-L")
            .arg("-o")
            .arg(&download_path)
            .arg(SOURCE_URL)
            .status()?
            .success()
        {
            bail!("Failed to download firefox source")
        }
    }

    Archive::new(XzDecoder::new(File::open(download_path)?)).unpack(&out_dir)?;

    let source_path = out_dir.join(format!("firefox-{FIREFOX_VERSION}"));

    for entry in fs::read_dir("patches")? {
        let path = entry?.path().canonicalize()?;

        if !Command::new("patch")
            .arg("-p0")
            .arg("--input")
            .arg(path)
            .current_dir(&source_path)
            .status()?
            .success()
        {
            bail!("Failed to patch spidermonkey")
        }
    }

    fs::create_dir_all(&build_path)?;

    if !Command::new(source_path.join("js/src/configure"))
        .args([
            "--disable-jemalloc",
            "--disable-js-shell",
            "--disable-shared-js",
            "--disable-export-js",
            "--disable-tests",
            "--enable-debug",
        ])
        .current_dir(&build_path)
        .status()?
        .success()
    {
        bail!("Failed to configure spidermonkey")
    }

    if !Command::new("make")
        .arg("-j")
        .arg(available_parallelism().unwrap().to_string())
        .current_dir(&build_path)
        .status()?
        .success()
    {
        bail!("Failed to configure spidermonkey")
    }

    File::create(out_dir.join(".monkey-ok"))?;

    Ok(build_path)
}

// Adapted from from rusty_v8
fn build_dir() -> PathBuf {
    let cwd = std::env::current_dir().unwrap();

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let out_dir_abs = cwd.join(out_dir);

    // This would be target/debug or target/release
    out_dir_abs
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

const ALLOWLIST_TYPES: &[&str] = &[
    "JSClass",
    "JSObject",
    "JSAutoRealm",
    "JSScript",
    "JS::OnNewGlobalHookOption",
    "JS::SelfHostedCache",
    "JS::SelfHostedWriter",
    "JS::Value",
    "JS::Realm",
    "js::GenericPrinter",
    "js::JSONPrinter",
];

const ALLOWLIST_VARS: &[&str] = &[
    "JSCLASS_GLOBAL_FLAGS",
    "JS::DefaultHeapMaxBytes",
    "JS::DefaultGlobalClassOps",
];

const ALLOWLIST_FUNCTION: &[&str] = &[
    "JS_NewContext",
    "JS_DestroyContext",
    "JS_ShutDown",
    "JS_NewGlobalObject",
    "JS::EnterRealm",
    "JS::LeaveRealm",
    "JS::InitSelfHostedCode",
];

const THREAD_SAFE_TYPES: &[&str] = &["JSClass"];

fn generate_bindings<P: AsRef<Path>>(path: P) -> Result<()> {
    let codegen_config =
        CodegenConfig::default() - CodegenConfig::CONSTRUCTORS - CodegenConfig::DESTRUCTORS;

    let mut bindings_builder = bindgen::Builder::default()
        .with_codegen_config(codegen_config)
        .header("src/spidermonkey.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args([
            format!("-I{}", path.as_ref().join("dist/include").display()).as_str(),
            "-std=c++20",
            "-w",
            "-x",
            "c++",
            "-DDEBUG=1",
        ])
        .enable_cxx_namespaces()
        .allowlist_recursively(false);

    for ty in ALLOWLIST_TYPES {
        bindings_builder = bindings_builder.allowlist_type(ty)
    }

    for var in ALLOWLIST_VARS {
        bindings_builder = bindings_builder.allowlist_var(var)
    }

    for fun in ALLOWLIST_FUNCTION {
        bindings_builder = bindings_builder.allowlist_function(fun)
    }

    for ty in THREAD_SAFE_TYPES {
        bindings_builder =
            bindings_builder.raw_line(format!("unsafe impl Sync for root::{ty} {{}}"));
    }

    bindings_builder = bindings_builder.default_enum_style(EnumVariation::Rust {
        non_exhaustive: true,
    });

    bindings_builder =
        bindings_builder.module_raw_line("root", "pub use crate::bindings::root::*;");

    bindings_builder =
        bindings_builder.module_raw_line("root::JS", "pub use crate::bindings::JS::*;");

    bindings_builder =
        bindings_builder.module_raw_line("root::js", "pub use crate::bindings::js::*;");

    let bindings = bindings_builder
        .generate()
        .context("Unable to generate bindings")?;

    let out_path = PathBuf::from(std::env::var("OUT_DIR").expect("No out dir"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .context("Couldn't write bindings!")?;

    Ok(())
}

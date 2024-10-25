use std::path::PathBuf;

use anyhow::{Context, Result};

const ALLOWLIST_TYPES: &[&str] = &[
    "JSClass",
    "JSClassOps",
    "JSContext",
    "JSRuntime",
    "JSPrincipals",
    "JSObject",
    "JSAutoRealm",
    "JSScript",
    "JS::OnNewGlobalHookOption",
    "JS::SelfHostedCache",
    "JS::SelfHostedWriter",
    "JS::Value",
    "JS::Realm",
    // "JS::CompileOptions",
    "js::ClassSpec",
    "js::ClassExtension",
    "js::ObjectOps",
    "js::GenericPrinter",
    "js::JSONPrinter",
];

const OPAQUE_TYPES: &[&str] = &[
    "JSClassOps",
    "JSContext",
    "JSRuntime",
    "JSPrincipals",
    "js::ClassSpec",
    "js::ClassExtension",
    "js::ObjectOps",
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
    "JS::InitSelfHostedCode",
];

const THREAD_SAFE_TYPES: &[&str] = &["JSClass"];

const CXX_TYPES: &[&str] = &["RealmOptions"];

fn main() -> Result<()> {
    let mut bindings_builder = bindgen::Builder::default()
        .header("src/spidermonkey.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args([
            "-I/usr/include/mozjs-128",
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

    for ty in OPAQUE_TYPES {
        bindings_builder = bindings_builder.opaque_type(ty)
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

    for ty in CXX_TYPES {
        bindings_builder =
            bindings_builder.module_raw_line("root::JS", format!("pub use crate::{ty};"));
    }

    let bindings = bindings_builder
        .generate()
        .context("Unable to generate bindings")?;

    let out_path = PathBuf::from(std::env::var("OUT_DIR").expect("No out dir"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .context("Couldn't write bindings!")?;

    cxx_build::bridge("src/lib.rs")
        .file("src/spidermonkey.hpp")
        .file("src/spidermonkey.cpp")
        .flag("-DDEBUG=1")
        .cpp(true)
        .std("c++20")
        .flag("-w")
        .include("/usr/include/mozjs-128")
        .compile("spidermonkey-sys");

    // Link to spidermonkey. The shared library is named mozjs-<version>
    // FIXME: Build and link statically
    pkg_config::Config::new()
        .probe("mozjs-128")
        .context("Failed to link mozjs")?;

    Ok(())
}

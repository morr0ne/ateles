#[cxx::bridge]
mod root {
    extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

        type JSClassOps;
        type JSContext;
        type JSRuntime;
        type JSPrincipals;

        unsafe fn JS_Init() -> bool;
    }
}

pub use root::*;

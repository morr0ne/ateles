#[cxx::bridge]
mod JS {
    #[namespace = "JS"]
    extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

        type CompileOptions;
        type RealmOptions;
    }

    extern "C++" {
        type JSContext = crate::JSContext;

        unsafe fn realm_options_new() -> UniquePtr<RealmOptions>;
        unsafe fn compile_options_new(cx: *mut JSContext) -> UniquePtr<CompileOptions>;
    }
}

pub use JS::*;

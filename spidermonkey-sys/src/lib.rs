pub mod sys;

use cxx::{type_id, ExternType};

unsafe impl ExternType for sys::root::JSContext {
    type Id = type_id!("JSContext");

    type Kind = cxx::kind::Opaque;
}

unsafe impl ExternType for sys::root::JSRuntime {
    type Id = type_id!("JSRuntime");

    type Kind = cxx::kind::Opaque;
}

unsafe impl ExternType for sys::root::JSClass {
    type Id = type_id!("JSClass");

    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
mod ffi {

    extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

        type JSContext = crate::sys::root::JSContext;
        type JSRuntime = crate::sys::root::JSRuntime;
        type JSClass = crate::sys::root::JSClass;

        unsafe fn JS_Init() -> bool;
    }

    unsafe extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

        fn realm_options_new() -> UniquePtr<RealmOptions>;
    }

    #[namespace = "JS"]
    extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

        type RealmOptions;
    }
}

pub use ffi::*;

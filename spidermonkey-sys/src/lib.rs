extern crate encoding_c;
extern crate encoding_c_mem;
extern crate icu_capi;
extern crate libz_sys;

pub mod sys;

// use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {

    extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

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

#[no_mangle]
pub extern "C" fn install_rust_hooks() {
    // FIXME: figure out if we need to do something in here
}

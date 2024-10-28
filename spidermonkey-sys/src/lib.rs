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

        type JSClassOps;
        type JSContext;
        type JSRuntime;
        type JSPrincipals;

        unsafe fn JS_Init() -> bool;
    }

    unsafe extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

        fn realm_options_new() -> UniquePtr<RealmOptions>;
    }

    #[namespace = "JS"]
    extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

        type CompileOptions;
        type RealmOptions;
    }

    #[namespace = "js"]
    extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

        type ClassSpec;
        type ClassExtension;
        type ObjectOps;

    }
}

pub use ffi::*;

#[no_mangle]
pub extern "C" fn install_rust_hooks() {
    // FIXME: figure out if we need to do something in here
}

impl sys::root::JSAutoRealm {
    pub fn new(cx: *mut sys::root::JSContext, target: *mut sys::root::JSObject) -> Self {
        Self {
            cx_: cx,
            oldRealm_: unsafe { sys::root::JS::EnterRealm(cx, target) },
        }
    }
}

impl Drop for sys::root::JSAutoRealm {
    fn drop(&mut self) {
        unsafe {
            sys::root::JS::LeaveRealm(self.cx_, self.oldRealm_);
        }
    }
}

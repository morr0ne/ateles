extern crate encoding_c;
extern crate encoding_c_mem;
extern crate icu_capi;
extern crate libz_sys;

mod bindings;
mod sys;

pub use sys::root::*;

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn install_rust_hooks() {
    // FIXME: figure out if we need to do something in here
}

impl JSAutoRealm {
    pub fn new(cx: *mut JSContext, target: *mut JSObject) -> Self {
        Self {
            cx_: cx,
            oldRealm_: unsafe { JS::EnterRealm(cx, target) },
        }
    }
}

impl Drop for JSAutoRealm {
    fn drop(&mut self) {
        unsafe {
            JS::LeaveRealm(self.cx_, self.oldRealm_);
        }
    }
}

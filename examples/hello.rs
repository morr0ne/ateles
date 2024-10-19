use std::ptr::{null, null_mut};

use anyhow::{bail, Result};
use spidermonkey_sys::{
    JSClass, JS_DestroyContext, JS_Init, JS_NewContext, JS_ShutDown, JS, JSCLASS_GLOBAL_FLAGS,
};

// const JSClassOps JS::DefaultGlobalClassOps = {
//     nullptr,                         // addProperty
//     nullptr,                         // delProperty
//     nullptr,                         // enumerate
//     JS_NewEnumerateStandardClasses,  // newEnumerate
//     JS_ResolveStandardClass,         // resolve
//     JS_MayResolveStandardClass,      // mayResolve
//     nullptr,                         // finalize
//     nullptr,                         // call
//     nullptr,                         // construct
//     JS_GlobalObjectTraceHook,        // trace
// };

static GLOBAL_CLASS: JSClass = JSClass {
    name: c"Global".as_ptr(),
    flags: JSCLASS_GLOBAL_FLAGS,
    cOps: null(),
    spec: null(),
    ext: null(),
    oOps: null(),
};

fn main() -> Result<()> {
    unsafe {
        if !JS_Init() {
            bail!("Failed to init runtime")
        }

        let cx = JS_NewContext(JS::DefaultHeapMaxBytes, null_mut());

        if cx.is_null() {
            bail!("Failed to create new context")
        }

        let options = JS::RealmOptions::new();

        JS_DestroyContext(cx);
        JS_ShutDown();
    }

    Ok(())
}

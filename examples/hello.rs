use std::ptr::{null, null_mut};

use anyhow::{bail, Result};
use spidermonkey_sys::{
    JSAutoRealm, JSClass, JS_DestroyContext, JS_Init, JS_NewContext, JS_NewGlobalObject,
    JS_ShutDown, JS, JSCLASS_GLOBAL_FLAGS,
};

static GLOBAL_CLASS: JSClass = JSClass {
    name: c"Global".as_ptr(),
    flags: JSCLASS_GLOBAL_FLAGS,
    cOps: unsafe { &JS::DefaultGlobalClassOps },
    spec: null(),
    ext: null(),
    oOps: null(),
};

fn main() -> Result<()> {
    let code = "(`hello world, it is ${new Date()}`)";

    unsafe {
        if !JS_Init() {
            bail!("Failed to init runtime")
        }

        let cx = JS_NewContext(JS::DefaultHeapMaxBytes, null_mut());

        if cx.is_null() {
            bail!("Failed to create new context")
        }

        if !JS::InitSelfHostedCode(cx, Default::default(), None) {
            bail!("Failed to init self hosted code")
        }

        let options = JS::realm_options_new();

        // FIXME: This should be rooted
        let global = JS_NewGlobalObject(
            cx,
            &GLOBAL_CLASS,
            null_mut(),
            JS::OnNewGlobalHookOption::FireOnNewGlobalHook,
            &*options,
        );

        let ar = JSAutoRealm::new(cx, global);

        let compile_options = JS::compile_options_new(cx);

        drop(ar);

        JS_DestroyContext(cx);
        JS_ShutDown();
    }

    Ok(())
}

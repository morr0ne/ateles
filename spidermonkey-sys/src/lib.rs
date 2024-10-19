use autocxx::prelude::*;

include_cpp! {
    #include "spidermonkey.hpp"
    // safety!(unsafe_ffi)
    generate!("JS_Init")
    generate!("JS_NewContext")
    generate!("JS::DefaultHeapMaxBytes")
    generate!("JS_DestroyContext")
    generate!("JS_ShutDown")
    generate!("JS::RealmOptions")
    generate_pod!("JSClass")
    generate!("JSCLASS_GLOBAL_FLAGS")
    generate!("JS::DefaultGlobalClassOps")
    // generate!("JS_NewEnumerateStandardClasses")
    // generate!("JS_ResolveStandardClass")
    // generate!("JS_MayResolveStandardClass")
    // generate!("JS_GlobalObjectTraceHook")
}

unsafe impl Sync for JSClass {}

pub use ffi::*;

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

use autocxx::prelude::*;

include_cpp! {
    #include "spidermonkey.hpp"
    // safety!(unsafe_ffi)
    generate!("JS_Init")
}

pub use ffi::*;

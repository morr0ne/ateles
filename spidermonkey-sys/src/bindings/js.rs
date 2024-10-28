#[cxx::bridge]
mod js {
    #[namespace = "js"]
    extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

        type ClassSpec;
        type ClassExtension;
        type ObjectOps;
    }
}

pub use js::*;

#[cxx::bridge]
pub mod root {
    extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

        type JSClassOps;
        type JSContext;
        type JSRuntime;
        type JSPrincipals;

        unsafe fn JS_Init() -> bool;
    }
}

#[cxx::bridge]
pub mod JS {
    #[namespace = "JS"]
    extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

        type CompileOptions;
        type RealmOptions;
    }

    unsafe extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

        fn realm_options_new() -> UniquePtr<RealmOptions>;
    }
}

#[cxx::bridge]
pub mod js {
    #[namespace = "js"]
    extern "C++" {
        include!("spidermonkey-sys/src/spidermonkey.hpp");

        type ClassSpec;
        type ClassExtension;
        type ObjectOps;
    }
}

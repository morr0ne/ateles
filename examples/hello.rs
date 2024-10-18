use anyhow::{bail, Result};
use spidermonkey_sys::JS_Init;

fn main() -> Result<()> {
    unsafe {
        if !JS_Init() {
            bail!("Failed to init runtime")
        }
    }

    Ok(())
}

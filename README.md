#### Cargo.toml
```toml
[dependencies]
tarantool = "0.4.2" # (1)
tarantool-test = { git = "https://github.com/chertov/tarantool-test" }
tarantool-test-macro = { git = "https://github.com/chertov/tarantool-test" }
```

#### lib.rs
```rust
#[macro_use] extern crate tarantool_test_macro;

#[tnt_test]
pub fn test_ok_void() {}

#[tnt_test]
pub fn test_res_ok() -> Result<(), anyhow::Error> {
    Ok(())
}

#[tnt_test]
pub fn test_res_err() -> Result<(), anyhow::Error> {
    Err(anyhow!("Test with Result Error"))
}

#[tnt_test]
pub fn test_res_panic() -> Result<(), anyhow::Error> {
    assert_eq!(3,2);
    Ok(())
}
#[tnt_test]
pub fn test_panic_void() {
    assert_eq!(3,2);
}

use std::os::raw::c_int;
use tarantool::tuple::{FunctionArgs, FunctionCtx};

#[no_mangle]
pub extern "C" fn rust_tests(_: FunctionCtx, _: FunctionArgs) -> c_int {
    // run all your tests inside the tarantool thread only!
    tarantool_test::run();
    0
}
```

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

#[tnt_test(init_db)] // the test requires to call init_db hook
pub fn test_void_ok() {}

#[tnt_test]
pub fn test_res_ok() -> Result<(), anyhow::Error> {
    Ok(())
}

#[tnt_test]
pub fn test_void_panic() {
    assert_eq!(3,2);
}

#[tnt_test(init_db)]
pub fn test_res_panic() -> Result<(), anyhow::Error> {
    assert_eq!(3,2);
    Ok(())
}

#[tnt_test]
pub fn test_res_err() -> Result<(), anyhow::Error> {
    Err(anyhow!("Test with Result Error"))
}

use std::os::raw::c_int;
use tarantool::tuple::{FunctionArgs, FunctionCtx};

#[no_mangle]
pub extern "C" fn rust_tests(_: FunctionCtx, _: FunctionArgs) -> c_int {
    tarantool_test::set_init_db_hook(|test| {
        debug!("init_db hook {:?}", test);
    });
    tarantool_test::set_test_start_hook(|test| {
        debug!("test_start hook {:?}", test);
    });
    tarantool_test::set_test_end_hook(|result| {
        debug!("test_end hook {:?}", result);
    });
    
    // run all your tests inside the tarantool thread only!
    let results = tarantool_test::run();
    debug!("results {:?}", results);
    0
}
```

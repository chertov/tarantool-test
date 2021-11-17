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
        debug!("init_db hook {:#?}", test);
    });
    tarantool_test::set_test_start_hook(|test| {
        debug!("test_start hook {:#?}", test);
    });
    tarantool_test::set_test_end_hook(|result| {
        debug!("test_end hook {:#?}", result);
    });
    
    // run all your tests inside the tarantool thread only!
    let results = tarantool_test::run();
    0
}
```

#### output
```
2021-11-17 21:29:57,484 DEBUG [app_tarantool::tarantool] init_db hook TestInfo {
    init_db: true,
    name: "test_void_ok",
    path: "app_tarantool::tarantool",
    func: Void(
        0x000000012c104e40,
    ),
}
2021-11-17 21:29:57,485 DEBUG [app_tarantool::tarantool] test_start hook TestInfo {
    init_db: true,
    name: "test_void_ok",
    path: "app_tarantool::tarantool",
    func: Void(
        0x000000012c104e40,
    ),
}
2021-11-17 21:29:57,485 DEBUG [tarantool_test] test [app_tarantool::tarantool::test_void_ok] is ok in 5us
2021-11-17 21:29:57,485 DEBUG [app_tarantool::tarantool] test_end hook TestResult {
    info: TestInfo {
        init_db: true,
        name: "test_void_ok",
        path: "app_tarantool::tarantool",
        func: Void(
            0x000000012c104e40,
        ),
    },
    res: None,
    duration: 5.923µs,
}
2021-11-17 21:29:57,485 DEBUG [app_tarantool::tarantool] test_start hook TestInfo {
    init_db: false,
    name: "test_res_ok",
    path: "app_tarantool::tarantool",
    func: Result(
        0x000000012c104e90,
    ),
}
2021-11-17 21:29:57,485 DEBUG [tarantool_test] test [app_tarantool::tarantool::test_res_ok] is ok in 4us
2021-11-17 21:29:57,485 DEBUG [app_tarantool::tarantool] test_end hook TestResult {
    info: TestInfo {
        init_db: false,
        name: "test_res_ok",
        path: "app_tarantool::tarantool",
        func: Result(
            0x000000012c104e90,
        ),
    },
    res: Some(
        Ok(
            (),
        ),
    ),
    duration: 4.212µs,
}
2021-11-17 21:29:57,485 DEBUG [app_tarantool::tarantool] test_start hook TestInfo {
    init_db: false,
    name: "test_void_panic",
    path: "app_tarantool::tarantool",
    func: Void(
        0x000000012c104ef0,
    ),
}
2021-11-17 21:29:57,488 ERROR [tarantool_test] test [app_tarantool::tarantool::test_void_panic] is failed in 138us! err: panicked at 'assertion failed: `(left == right)`
  left: `3`,
 right: `2`', src/tarantool.rs:15:1

Stack backtrace:
   0: <unknown>
   ...
  16: _coro_init

Stack backtrace:
   0: <unknown>
   ...
  15: _coro_init
2021-11-17 21:29:57,488 DEBUG [app_tarantool::tarantool] test_end hook TestResult {
    info: TestInfo {
        init_db: false,
        name: "test_void_panic",
        path: "app_tarantool::tarantool",
        func: Void(
            0x000000012c104ef0,
        ),
    },
    res: Some(
        Err(
            "panicked at 'assertion failed: `(left == right)`\n  left: `3`,\n right: `2`', src/tarantool.rs:15:1\n\nStack backtrace:\n   0: <unknown>\n   1: <unknown>\n   2: <unknown>\n   3: <unknown>\n   4: <unknown>\n   5: <unknown>\n   6: <unknown>\n   7: _module_func_call\n   8: _func_c_call\n   9: _func_call\n  10: _box_process_call\n  11: __ZL15tx_process_callP4cmsg\n  12: _cmsg_deliver\n  13: _fiber_pool_f\n  14: __ZL16fiber_cxx_invokePFiP13__va_list_tagES0_\n  15: _fiber_loop\n  16: _coro_init\n\nStack backtrace:\n   0: <unknown>\n   1: <unknown>\n   2: <unknown>\n   3: <unknown>\n   4: <unknown>\n   5: <unknown>\n   6: _module_func_call\n   7: _func_c_call\n   8: _func_call\n   9: _box_process_call\n  10: __ZL15tx_process_callP4cmsg\n  11: _cmsg_deliver\n  12: _fiber_pool_f\n  13: __ZL16fiber_cxx_invokePFiP13__va_list_tagES0_\n  14: _fiber_loop\n  15: _coro_init",
        ),
    ),
    duration: 138.075µs,
}
2021-11-17 21:29:57,488 DEBUG [app_tarantool::tarantool] init_db hook TestInfo {
    init_db: true,
    name: "test_res_panic",
    path: "app_tarantool::tarantool",
    func: Result(
        0x000000012c104f80,
    ),
}
2021-11-17 21:29:57,488 DEBUG [app_tarantool::tarantool] test_start hook TestInfo {
    init_db: true,
    name: "test_res_panic",
    path: "app_tarantool::tarantool",
    func: Result(
        0x000000012c104f80,
    ),
}
2021-11-17 21:29:57,489 ERROR [tarantool_test] test [app_tarantool::tarantool::test_res_panic] is failed in 89us! err: panicked at 'assertion failed: `(left == right)`
  left: `3`,
 right: `2`', src/tarantool.rs:20:1

Stack backtrace:
   0: <unknown>
   ...
  16: _coro_init

Stack backtrace:
   0: <unknown>
   ...
  15: _coro_init
2021-11-17 21:29:57,489 DEBUG [app_tarantool::tarantool] test_end hook TestResult {
    info: TestInfo {
        init_db: true,
        name: "test_res_panic",
        path: "app_tarantool::tarantool",
        func: Result(
            0x000000012c104f80,
        ),
    },
    res: Some(
        Err(
            "panicked at 'assertion failed: `(left == right)`\n  left: `3`,\n right: `2`', src/tarantool.rs:20:1\n\nStack backtrace:\n   0: <unknown>\n   1: <unknown>\n   2: <unknown>\n   3: <unknown>\n   4: <unknown>\n   5: <unknown>\n   6: <unknown>\n   7: _module_func_call\n   8: _func_c_call\n   9: _func_call\n  10: _box_process_call\n  11: __ZL15tx_process_callP4cmsg\n  12: _cmsg_deliver\n  13: _fiber_pool_f\n  14: __ZL16fiber_cxx_invokePFiP13__va_list_tagES0_\n  15: _fiber_loop\n  16: _coro_init\n\nStack backtrace:\n   0: <unknown>\n   1: <unknown>\n   2: <unknown>\n   3: <unknown>\n   4: <unknown>\n   5: <unknown>\n   6: _module_func_call\n   7: _func_c_call\n   8: _func_call\n   9: _box_process_call\n  10: __ZL15tx_process_callP4cmsg\n  11: _cmsg_deliver\n  12: _fiber_pool_f\n  13: __ZL16fiber_cxx_invokePFiP13__va_list_tagES0_\n  14: _fiber_loop\n  15: _coro_init",
        ),
    ),
    duration: 89.487µs,
}
2021-11-17 21:29:57,490 DEBUG [app_tarantool::tarantool] test_start hook TestInfo {
    init_db: false,
    name: "test_res_err",
    path: "app_tarantool::tarantool",
    func: Result(
        0x000000012c105010,
    ),
}
2021-11-17 21:29:57,490 ERROR [tarantool_test] test [app_tarantool::tarantool::test_res_err] is failed in 78us! err: Test with Result Error

Stack backtrace:
   0: <unknown>
   ...
  22: _coro_init
2021-11-17 21:29:57,490 DEBUG [app_tarantool::tarantool] test_end hook TestResult {
    info: TestInfo {
        init_db: false,
        name: "test_res_err",
        path: "app_tarantool::tarantool",
        func: Result(
            0x000000012c105010,
        ),
    },
    res: Some(
        Err(
            "Test with Result Error\n\nStack backtrace:\n   0: <unknown>\n   1: <unknown>\n   2: <unknown>\n   3: <unknown>\n   4: <unknown>\n   5: <unknown>\n   6: <unknown>\n   7: <unknown>\n   8: <unknown>\n   9: <unknown>\n  10: <unknown>\n  11: <unknown>\n  12: <unknown>\n  13: _module_func_call\n  14: _func_c_call\n  15: _func_call\n  16: _box_process_call\n  17: __ZL15tx_process_callP4cmsg\n  18: _cmsg_deliver\n  19: _fiber_pool_f\n  20: __ZL16fiber_cxx_invokePFiP13__va_list_tagES0_\n  21: _fiber_loop\n  22: _coro_init",
        ),
    ),
    duration: 78.212µs,
}
```
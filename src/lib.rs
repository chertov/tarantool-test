type TestFunctionVoid = fn();
type TestFunctionResult = fn() -> Result<(), anyhow::Error>;

#[derive(Debug, Clone)]
enum TestFunction {
    Void(TestFunctionVoid),
    Result(TestFunctionResult),
}

#[derive(Debug, Clone)]
pub struct TestInfo {
    pub name: String,
    pub path: String,
    func: TestFunction,
}

pub struct TestResult{
    pub info: TestInfo,
    pub res: Option<Result<(), anyhow::Error>>,
    pub duration: std::time::Duration,
}

static TESTS: once_cell::sync::Lazy<parking_lot::RwLock<std::collections::BTreeMap<String, Vec<TestInfo>>>> = once_cell::sync::Lazy::new(|| {
    parking_lot::RwLock::new(std::collections::BTreeMap::new())
});

pub fn run() -> std::collections::BTreeMap<String, Vec<TestResult>> {
    let mut modules = std::collections::BTreeMap::new();
    let tests = TESTS.read().clone();
    for (module_path, module_tests) in tests {
        let mut results = vec![];
        for info in module_tests {
            let now = std::time::Instant::now();

            let res = catch_unwind_silent(|| {
                match info.func {
                    TestFunction::Void(f) => { f(); None },
                    TestFunction::Result(f) => Some(f()),
                }
            });

            let duration = now.elapsed();
            let duration_str = duration_to_str(duration);

            let res = match res {
                Ok(res) => res,
                Err(err) => Some(Err(anyhow::anyhow!("{:?}", err))),
            };
            match &res {
                Some(Ok(())) | None => log::debug!("test [{}::{}] is ok in {}", info.path, info.name, duration_str),
                Some(Err(err)) => {
                    log::error!("test [{}::{}] is failed in {}! err: {:?}", info.path, info.name, duration_str, err);
                },
            };
            results.push(TestResult{info, res, duration})
        }
        modules.insert(module_path, results);
    }
    modules
}


pub fn __collect_test_void(path: &str, name: &str, func: TestFunctionVoid) {
    collect_test(path, name, TestFunction::Void(func))
}
pub fn __collect_test_with_result(path: &str, name: &str, func: TestFunctionResult) {
    collect_test(path, name, TestFunction::Result(func))
}

fn collect_test(path: &str, name: &str, func: TestFunction) {
    let path = path.to_string();
    let name = name.to_string();
    {
        if let Some(tests) = TESTS.write().get_mut(&path) {
            tests.push(TestInfo{path, name, func});
            return;
        }
    }
    TESTS.write().insert(path.clone(), vec![TestInfo{path, name, func}]);
}


fn catch_unwind_silent<F: FnOnce() -> R + std::panic::UnwindSafe, R>(f: F) -> Result<R, anyhow::Error> {
    let prev_hook = std::panic::take_hook();
    let panic_err = std::sync::Arc::new(parking_lot::RwLock::new("".to_string()));
    std::panic::set_hook(Box::new({
        let panic_err = panic_err.clone();
        move |err| { *panic_err.write() = err.to_string(); }
    }));
    let result = std::panic::catch_unwind(f);
    std::panic::set_hook(prev_hook);
    match result {
        Ok(res) => Ok(res),
        Err(_) => Err(anyhow::anyhow!("{}", panic_err.read().clone())),
    }
}

fn duration_to_str(d: std::time::Duration) -> String {
    if d.as_micros() < 10_000 { return format!("{}us", d.as_micros()) }
    if d.as_millis() < 1000 { return format!("{}ms", d.as_millis()) }
    if d.as_secs() < 60 { return format!("{}s", d.as_secs()) }

    match chrono::Duration::from_std(d) {
        Ok(d) => format!("{}", chrono_humanize::HumanTime::from(d)),
        Err(_) => format!("{}min", d.as_secs_f64() / 60.0),
    }
}

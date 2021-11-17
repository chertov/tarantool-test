type TestFunctionVoid = fn();
type TestFunctionResult = fn() -> Result<(), anyhow::Error>;

#[derive(Debug, Clone)]
enum TestFunction {
    Void(TestFunctionVoid),
    Result(TestFunctionResult),
}

#[derive(Debug, Clone)]
pub struct TestInfo {
    pub init_db: bool,
    pub name: String,
    pub path: String,
    func: TestFunction,
}

#[derive(Debug)]
pub struct TestResult {
    pub info: TestInfo,
    pub res: Option<Result<(), anyhow::Error>>,
    pub duration: std::time::Duration,
}
impl Clone for TestResult {
    fn clone(&self) -> Self {
        let res = match &self.res {
            Some(Ok(())) => Some(Ok(())),
            Some(Err(err)) => Some(Err(anyhow::anyhow!("{:?}", err))),
            None => None,
        };
        Self {
            info: self.info.clone(),
            duration: self.duration,
            res
        }
    }
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

            if info.init_db {
                INIT_DB.read().as_ref().map(|init_db| init_db(info.clone()));
            }
            START.read().as_ref().map(|start| start(info.clone()));
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
            let result = TestResult{info, res, duration};

            END.read().as_ref().map(|end| end(result.clone()));

            results.push(result)
        }
        modules.insert(module_path, results);
    }
    modules
}

pub fn set_init_db_hook(init_db: impl Fn(TestInfo) + 'static  + Send + Sync) {
    INIT_DB.write().replace(Box::new(init_db));
}
pub fn set_test_start_hook(callback: impl Fn(TestInfo) + 'static  + Send + Sync) {
    START.write().replace(Box::new(callback));
}
pub fn set_test_end_hook(callback: impl Fn(TestResult) + 'static  + Send + Sync) {
    END.write().replace(Box::new(callback));
}

static INIT_DB: once_cell::sync::Lazy<parking_lot::RwLock<Option<Box<dyn Fn(TestInfo) + 'static  + Send + Sync>>>> = once_cell::sync::Lazy::new(|| parking_lot::RwLock::new(None));
static START: once_cell::sync::Lazy<parking_lot::RwLock<Option<Box<dyn Fn(TestInfo) + 'static + Send + Sync>>>> = once_cell::sync::Lazy::new(|| parking_lot::RwLock::new(None));
static END: once_cell::sync::Lazy<parking_lot::RwLock<Option<Box<dyn Fn(TestResult) + 'static + Send + Sync>>>> = once_cell::sync::Lazy::new(|| parking_lot::RwLock::new(None));

pub fn __collect_test_void(path: &str, name: &str, func: TestFunctionVoid, init_db: bool) {
    collect_test(path, name, TestFunction::Void(func), init_db)
}
pub fn __collect_test_with_result(path: &str, name: &str, func: TestFunctionResult, init_db: bool) {
    collect_test(path, name, TestFunction::Result(func), init_db)
}

fn collect_test(path: &str, name: &str, func: TestFunction, init_db: bool) {
    let path = path.to_string();
    let name = name.to_string();
    {
        if let Some(tests) = TESTS.write().get_mut(&path) {
            tests.push(TestInfo{ init_db, path, name, func });
            return;
        }
    }
    TESTS.write().insert(path.clone(), vec![TestInfo{ init_db, path, name, func }]);
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

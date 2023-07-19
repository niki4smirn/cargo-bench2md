use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum Message {
    Suite(SuiteMessage),
    Test(TestMessage),
    Bench(BenchOutcome),
}

#[derive(Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub(crate) enum SuiteMessage {
    Ok(SuiteOutcome),
    Failed(SuiteOutcome),
    Started { test_count: usize },
}

#[derive(Deserialize)]
pub(crate) struct SuiteOutcome {
    passed: usize,
    failed: usize,
    ignored: usize,
    measured: usize,
    filtered_out: usize,
    exec_time: f64,
}

#[derive(Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub(crate) enum TestMessage {
    Ok(TestOutcome),
    Failed(TestOutcome),
    Ignored(TestOutcome),
    Timeout { name: String },
    Started,
}

#[derive(Deserialize)]
pub(crate) struct BenchOutcome {
    pub name: String,
    pub median: u64,
    pub deviation: u64,
}

#[derive(Deserialize)]
pub(crate) struct TestOutcome {
    name: String,
    exec_time: Option<f64>,
    stdout: Option<String>,
    message: Option<String>,
}

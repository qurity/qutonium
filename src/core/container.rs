use crate::core::{hooker::Hook, tester::TestCase};


use std::{
  fmt::{Debug, Display, Formatter, Result},
  sync::{Arc, Mutex},
};


#[derive(Debug)]
pub struct State {
  pub failed: u64,
  pub passed: u64,
  pub hooks: Arc<Mutex<Vec<Hook>>>,
  pub test_cases: Arc<Mutex<Vec<TestCase>>>,
}


impl Display for State {
  fn fmt (&self, f: &mut Formatter) -> Result {
    f.debug_struct("State")
    .field("failed", &self.failed)
    .field("passed", &self.passed)
    .field("test_cases", &self.test_cases)
    .finish()
  }
}


impl State {
  pub fn new (hooks: Vec<Hook>, test_cases: Vec<TestCase>) -> Self {
    State {
      failed: 0,
      passed: 0,
      hooks: Arc::new(Mutex::new(hooks)),
      test_cases: Arc::new(Mutex::new(test_cases)),
    }
  }
}


#[macro_export]
macro_rules! state {
  ($hooks:expr, $test_cases:expr) => {{
    use std::sync::{Arc, Mutex};

    Arc::new(Mutex::new(State::new($hooks, $test_cases)))
  }};
}

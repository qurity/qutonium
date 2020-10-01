use crate::{
  core::{
    container::State,
    hooker::{Hook, HookType},
    reporter::Report,
    tester::{Closure, Test},
    timer::TimeUnitType,
  },
  exports::export::{Export, ExportConfig},
};


use std::{
  collections::HashMap,
  sync::{Arc, Mutex, MutexGuard},
};


use miniserde::Serialize;


#[derive(Debug)]
pub struct SuiteCase {
  hooks: HashMap<HookType, Box<dyn Closure>>,
  name: &'static str,
  result: Option<Suite>,
  state: Option<Arc<Mutex<State>>>,
  time_unit: TimeUnitType,
}


impl SuiteCase {
  pub fn new (name: &'static str) -> Self {
    SuiteCase {
      hooks: HashMap::new(),
      name,
      result: None,
      state: None,
      time_unit: TimeUnitType::Milliseconds,
    }
  }

  fn export (suite: Suite) {
    Export::new(ExportConfig {
      data: suite,
      filename: "test.json",
      pathname: "tmp",
    }).to_json();
  }

  pub fn render (&mut self) {
    self.render_suite();
  }

  pub fn render_hook (
    &mut self,
    hooks_mutex: MutexGuard<Vec<Hook>>,
    hook_type: HookType
  ) {
    hooks_mutex.iter().for_each(|h| { if hook_type == h.kind { (h.f)() }});
  }

  pub fn set_state (mut self, state: Arc<Mutex<State>>) -> Self {
    self.state = Some(state);
    self
  }

  // TODO: how to used mutex in a proper way?
  fn render_suite (&mut self) {
    let state_mutex = Arc::clone(&self.state.as_ref().unwrap());
    let mut state = state_mutex.lock().unwrap();
    let hooks_mutex = Arc::clone(&state.hooks);
    let test_cases_mutex = Arc::clone(&state.test_cases);
    let mut test_cases = test_cases_mutex.lock().unwrap();
    let mut suite = Suite::new(self.name.into(), self.time_unit);

    self.render_hook(hooks_mutex.lock().unwrap(), HookType::BeforeAll);

    for x in 0..test_cases.len() {
      let test = &mut test_cases[x];

      self.render_hook(hooks_mutex.lock().unwrap(), HookType::BeforeEach);
      test.render();

      if test.pass == true {
        state.passed += 1;
        suite.passed = state.passed;
      } else {
        state.failed += 1;
        suite.failed = state.failed;
      }

      suite.update_from_test(test.result());
      self.render_hook(hooks_mutex.lock().unwrap(), HookType::AfterEach);
    }

    suite.sum_tests();
    self.render_hook(hooks_mutex.lock().unwrap(), HookType::AfterAll);
    self.report_suite_results(suite);
  }

  fn report_suite_results (&mut self, suite: Suite) {
    Report::result(suite.clone());
    SuiteCase::export(suite.clone());
  }
}


#[derive(Debug, Serialize)]
pub struct Suite {
  pub failed: u64,
  pub name: String,
  pub passed: u64,
  pub tests: Vec<Test>,
  pub time_unit_type: TimeUnitType,
  pub total: u64,
}


impl Clone for Suite {
  fn clone (&self) -> Self {
    Suite {
      failed: self.failed.clone(),
      name: self.name.clone(),
      passed: self.passed.clone(),
      tests: self.tests.clone(),
      time_unit_type: self.time_unit_type.clone(),
      total: self.total.clone()
    }
  }
}


impl Suite {
  pub fn new (name: String, time_unit_type: TimeUnitType) -> Self {
    Suite {
      name,
      failed: 0,
      passed: 0,
      time_unit_type,
      tests: vec![],
      total: 0,
    }
  }

  pub fn sum_tests (&mut self) {
    self.total = self.passed + self.failed;
  }

  pub fn update_from_test (&mut self, test: Test) {
    self.tests.push(test)
  }
}


#[macro_export]
macro_rules! suite {
  ($sn:expr,
    {
      $($tcn:tt $tcc:expr)*
    }
  ) => {{
    let mut hooks: Vec<Hook> = vec![];
    let mut test_cases: Vec<TestCase> = vec![];

    $(test_cases.push(TestCase::new($tcn, $tcc));)*

    SuiteCase::new($sn).set_state(state!(hooks, test_cases)).render()
  }};
  ($sn:expr,
    {
      // hooks
      $(before:all $hba:expr,)?
      $(before:each $hbe:expr,)?
      $(after:all $haa:expr,)?
      $(after:each $hae:expr)?;
      // test cases
      $($tcn:tt $tcc:expr)*
    }) => {{
      let mut hooks: Vec<Hook> = vec![];
      let mut test_cases: Vec<TestCase> = vec![];

      $(hooks.push(Hook {
        f: Box::new($hba),
        kind: HookType::BeforeAll,
        name: "before:all",
      });)?

      $(hooks.push(Hook {
        f: Box::new($hbe),
        kind: HookType::BeforeEach,
        name: "before:each",
      });)?

      $(hooks.push(Hook {
        f: Box::new($haa),
        kind: HookType::AfterAll,
        name: "after:all",
      });)?

      $(hooks.push(Hook {
        f: Box::new($hae),
        kind: HookType::AfterEach,
        name: "after:each",
      });)?

      $(test_cases.push(TestCase::new($tcn, $tcc));)*

      SuiteCase::new($sn).set_state(state!(hooks, test_cases)).render()
  }};
  ($sn:expr,
    {
      // hooks ~sugar
      $(ba: $hba:expr,)?
      $(be: $hbe:expr,)?
      $(aa: $haa:expr,)?
      $(ae: $hae:expr)?;
      // test cases
      $($tcn:tt $tcc:expr)*
    }) => {
    {
      let mut hooks: Vec<Hook> = vec![];
      let mut test_cases: Vec<TestCase> = vec![];
      
      $(hooks.push(Hook {
        f: Box::new($hba),
        kind: HookType::BeforeAll,
        name: "before.all",
      });)?
      
      $(hooks.push(Hook {
        f: Box::new($hbe),
        kind: HookType::BeforeEach,
        name: "before.each",
      });)?
      
      $(hooks.push(Hook {
        f: Box::new($haa),
        kind: HookType::AfterAll,
        name: "after.all",
      });)?
      
      $(hooks.push(Hook {
        f: Box::new($hae),
        kind: HookType::AfterEach,
        name: "after.each",
      });)?

      $(test_cases.push(TestCase::new($tcn, $tcc));)*

      SuiteCase::new($sn).set_state(state!(hooks, test_cases)).render()
    }
  };
}

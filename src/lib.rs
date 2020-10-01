pub mod core;
pub mod exports;
pub mod matchers;
pub mod utils;


pub mod prelude {
  pub use crate::{
    core::{
      container::State,
      expecter::ExpectCase,
      hooker::{Hook, HookType},
      matcher::{Matcher, Matcherable},
      reporter::Report,
      sourcer::{Source, Location},
      styler::{
        style_label,
        style_result_error_label,
        style_result_success_label,
        style_suite_label,
        style_suite_stats_label,
        style_test_label,
      },
      suiter::{SuiteCase, Suite},
      tester::{Closure, ClosureResult, Test, TestCase},
      timer::{TimeUnitType, TimeUnit, Time}
    },
    exports::{export::{Export, ExportConfig}},
    matchers::{
      common::equal::be_equal,
      primitive::{
        boolean::{be_false, be_true},
        never::abort,
      },
    },
    expect, impl_matcher, must, state, suite,
  };
}

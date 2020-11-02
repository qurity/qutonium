use crate::core::{
  styler::{
    style_result_success_label,
    style_result_error_label,
    style_suite_label,
    style_test_label,
    style_suite_stats_label,
  },
  tester::Test,
  suiter::Suite,
};


use qute::prelude::*;


pub struct Report;


impl Report {
  pub fn print_suite_result<F> (suite: Suite, f: F)
  where
    F: FnOnce(Vec<Test>) + Sized
  {
    println!("\n{} {}", style_suite_label("SUITE"), suite.name);
    f(suite.tests);
    println!("{}", style_suite_label("END "));
  }

  pub fn print_suite_stats (suite: Suite) {
    let suite_stats_failed = String::from(
      format!("  👎 failed: {}", suite.failed)
    );

    let suite_stats_passed = String::from(
      format!("  👍 passed: {}", suite.passed)
    );

    let suite_stats_total = String::from(
      format!("  📈 total: {}", suite.total)
    );

    println!("\n{}", style_suite_stats_label("DIAGNOSTIC"));

    println!(
      "{} {} {}",
      qute!(&suite_stats_failed).set_css_color("red").bold(),
      qute!(&suite_stats_passed).set_css_color("lime").bold(),
      qute!(&suite_stats_total).set_css_color("cyan").bold(),
    );

    println!("{}", style_suite_stats_label("END"));
  }

  pub fn parse_test_diagnostic_result (test: &Test) -> String {
    let mut test_diagnostic_result = Vec::<String>::new();

    let test_diagnostic_location = String::from(format!(
      "\n       📍 {}:{}:{}",
      test.file, test.line, test.column
    ));

    if test.pass == true {
      let test_diagnostic_result_begin_label = String::from(
        format!("\n    {}", style_result_success_label("SUCCESS"))
      );

      let test_diagnostic_diagnostic = String::from(format!(
        "\n       👍 {}", test.diagnostic
      ));

      let test_diagnostic_result_end_label = String::from(format!(
        "\n    {}", style_result_success_label("END")
      ));

      test_diagnostic_result.push(test_diagnostic_result_begin_label);
      test_diagnostic_result.push(test_diagnostic_diagnostic);
      test_diagnostic_result.push(test_diagnostic_location);
      test_diagnostic_result.push(test_diagnostic_result_end_label);

      return test_diagnostic_result.join("")
    } else {
      let test_diagnostic_result_begin_label = String::from(format!(
        "\n    {}", style_result_error_label("ERROR")
      ));

      let test_diagnostic_diagnostic = String::from(format!(
        "\n       👎 {}", test.diagnostic
      ));

      let test_diagnostic_result_end_label = String::from(format!(
        "\n    {}", style_result_error_label("END")
      ));

      test_diagnostic_result.push(test_diagnostic_result_begin_label);
      test_diagnostic_result.push(test_diagnostic_diagnostic);
      test_diagnostic_result.push(test_diagnostic_location);
      test_diagnostic_result.push(test_diagnostic_result_end_label);

      return test_diagnostic_result.join("")
    }
  }



  pub fn print_test_result (test: &Test) {
    let mut test_result = Vec::<String>::new();
    let test_tests_result = Report::parse_test_diagnostic_result(&test);
    let test_begin_label = String::from(format!(
      "  {} ", style_test_label("TEST")
    ));

    let test_name = String::from(format!(
      "{}", test.name
    ));

    let test_time = String::from(format!(
      " ({}ms)", test.time_unit.millis.to_string()
    ));

    let test_end_label = String::from(format!(
      "\n  {}", style_test_label("END")
    ));

    test_result.push(test_begin_label);      
    test_result.push(test_name);
    test_result.push(test_time);
    test_result.push(test_tests_result);
    test_result.push(test_end_label);      

    println!("{}", test_result.join(""));

    test_result.clear();
  }

  pub fn print_tests_result (tests: Vec<Test>) {
    tests.iter().for_each(|t| Report::print_test_result(t));
  }

  pub fn result (suite: Suite) {
    if suite.tests.len() > 0 {
      Report::print_suite_result(suite.clone(), Report::print_tests_result);
      Report::print_suite_stats(suite.clone());
    }
  }
}

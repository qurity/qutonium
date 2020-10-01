use qute::prelude::style;


pub fn style_label (value: &'static str, color: &'static str) -> String {
  style!({
    value: value;
    color: color;
    background: "black";
    font-weight: none;
  })
}


pub fn style_suite_label (value: &'static str) -> String {
  style!({
    value: value;
    color: "floralwhite";
    background: "black";
    font-weight: none;
  })
}


pub fn style_test_label (value: &'static str) -> String {
  style!({
    value: value;
    color: "cyan";
    background: "black";
    font-weight: none;
  })
}


pub fn style_result_success_label (value: &'static str) -> String {
  style!({
    value: value;
    color: "lime";
    background: "black";
    font-weight: none;
  })
}


pub fn style_result_suite_label (value: &'static str) -> String {
  style!({
    value: value;
    color: "lightgrey";
    background: "black";
    font-weight: none;
  })
}


pub fn style_result_error_label (value: &'static str) -> String {
  style!({
    value: value;
    color: "crimson";
    background: "black";
    font-weight: none;
  })
}


pub fn style_suite_stats_label (value: &'static str) -> String {
  style!({
    value: value;
    color: "lightgrey";
    background: "black";
    font-weight: none;
  })
}

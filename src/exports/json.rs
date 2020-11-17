use crate::{
  core::suiter::Suite,
  exports::export::Exporter,
};


use miniserde::json;


pub struct JSON {
  pub output: String,
  pub pathname: String,
  data: Suite,
}


impl Exporter for JSON {
  fn new (data: Suite) -> Self {
    JSON {
      output: "".into(),
      pathname: "".into(),
      data,
    }
  }

  fn path (mut self, pathname: String) -> Self {
    self.pathname = pathname;
    self
  }

  fn parse (mut self) -> Self {
    self.output = json::to_string(&self.data);
    self
  }
}

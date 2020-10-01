use crate::{
  core::suiter::Suite,
  exports::export::Exporter,
};


use miniserde::json;


pub struct JSON {
  data: Suite,
  filename: &'static str,
  pathname: &'static str,
}


impl Exporter for JSON {
  fn new (data: Suite) -> Self {
    JSON {
      data,
      filename: "",
      pathname: "",
    }
  }

  fn name (mut self, filename: &'static str) -> Self {
    self.filename = filename;
    self
  }

  fn path (mut self, pathname: &'static str) -> Self {
    self.pathname = pathname;
    self
  }

  fn parse (self) -> String {
    json::to_string(&self.data)
  }
}

use crate::{
  core::{
    expecter::Expect,
    sourcer::Location,
    timer::{Time, TimeUnit}
  },
  utils::types,
};


use std::{fmt, time::Duration};


use miniserde::Serialize;


pub trait Closure: Fn() -> Result<Expect, Expect> + 'static {}
pub type ClosureResult = dyn Fn() -> Result<Expect, Expect> + 'static;


impl fmt::Debug for dyn Closure {
  fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", &types::get_type_name_by_type(&self))
  }
}


#[derive(Debug, Serialize)]
pub struct Test {
  pub column: u32,
  pub diagnostic: String,
  pub file: String,
  pub line: u32,
  pub name: String,
  pub pass: bool,
  pub time_unit: TimeUnit,
}


impl Test {
  pub fn new (
    column: u32,
    diagnostic: String,
    file: String,
    line: u32,
    name: String,
    pass: bool,
    time_unit: TimeUnit,
  ) -> Self {
    Test {
      column,
      diagnostic,
      file,
      line,
      name,
      pass,
      time_unit,
    }
  }
}


impl Clone for Test {
  fn clone (&self) -> Self {
    Test {
      column: self.column.clone(),
      diagnostic: self.diagnostic.clone(),
      file: self.file.clone(),
      line: self.line.clone(),
      name: self.name.clone(),
      pass: self.pass.clone(),
      time_unit: self.time_unit.clone(),
    }
  }
}


pub struct TestCase {
  pub test: Box<ClosureResult>,
  pub diagnostic: Option<String>,
  pub location: Option<Location>,
  pub name: &'static str,
  pub only: bool,
  pub pass: bool,
  pub skip: bool,
  pub time: Time,
}


impl fmt::Debug for TestCase {
  fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("TestCase")
    .field("test", &types::get_type_name_by_type(&self))
    .field("name", &self.name)
    .finish()
  }
}


impl TestCase {
  pub fn new<T> (name: &'static str, test: T) -> Self
  where
    T: Fn() -> Result<Expect, Expect> + 'static + Copy + Clone
  {
    TestCase {
      test: Box::new(test),
      diagnostic: None,
      location: None,
      name,
      only: false,
      pass: false,
      skip: false,
      time: Time::new(),
    }
  }

  pub fn only (mut self) -> Self {
    self.only = true;
    self
  }

  pub fn render (&mut self) {
    let test: &mut ClosureResult = &mut self.test;
    
    match self.time.start() {
      Some(start) => {
        self.time.start = Some(start);
      },
      _ => (),
    };

    match test() {
      Ok(result) => {
        self.pass = true;
        self.diagnostic = result.diagnostic.clone();
        self.location = result.location.clone();
      },
      Err(result) => {
        self.pass = false;
        self.diagnostic = result.diagnostic.clone();
        self.location = result.location.clone();
      },
    }

    match self.time.end() {
      Some(end) => {
        self.time.end = Some(end);
      },
      _ => (),
    }

    self.time.unit = TimeUnit {
      micros: self.time.end.unwrap_or(Duration::new(0,0)).as_micros() as u64,
      millis: self.time.end.unwrap_or(Duration::new(0,0)).as_millis() as u64,
      nanos: self.time.end.unwrap_or(Duration::new(0,0)).as_nanos() as u64,
      secs: self.time.end.unwrap_or(Duration::new(0,0)).as_secs() as u64,
    };
  }

  pub fn result (&self) -> Test {
    Test::new(
      self.location.unwrap().column,
      self.diagnostic.clone().unwrap(),
      self.location.unwrap().file.into(),
      self.location.unwrap().line,
      self.name.into(),
      self.pass,
      self.time.unit,
    )
  }
}

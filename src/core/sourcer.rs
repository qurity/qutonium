use std::fmt::{Display, Formatter, Result};


#[derive(Clone, Copy, Debug)]
pub struct Location {
  pub column: u32,
  pub file: &'static str,
  pub line: u32,
}


impl Location {
  pub fn new (file: &'static str, line: u32, column: u32) -> Self {
    Location { column, file, line }
  }
}


#[derive(Clone, Copy, Debug)]
pub struct Source {
  pub location: Location,
}


impl Display for Source {
  fn fmt (&self, f: &mut Formatter<'_>) -> Result {
    let Location { column, file, line } = self.location;

    write!(f, "{}:{}:{}", file, line, column)
  }
}


impl Source {
  pub fn new (location: Location) -> Self {
    Source { location }
  }
}

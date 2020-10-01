use std::fmt::Debug;


#[derive(Debug, PartialEq)]
pub enum Mark {
  Not,
  To,
}


impl Mark {
  pub fn is_assertion (&self) -> bool {
    *self == Mark::To
  }
}

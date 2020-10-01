use crate::core::matcher::{Matcher, Matcherable};


pub fn be_boolean () -> Matcherable<bool> {
  Matcherable::new(true)
}


pub fn be_false () -> Matcherable<bool> {
  Matcherable::new(false)
}


pub fn be_true () -> Matcherable<bool> {
  Matcherable::new(true)
}

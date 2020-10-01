use crate::core::matcher::{Matcher, Matcherable};


pub fn abort () -> Matcherable<bool> {
  Matcherable::new(true)
}

use crate::core::matcher::{Matcher, Matcherable};


use std::fmt::Debug;


pub fn be_equal<V> (value: V) -> Matcherable<V>
where
  V: Clone + Copy + Debug + PartialEq + Sized + Sync
{
  Matcherable::new(value)
}

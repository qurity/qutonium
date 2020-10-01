use crate::{
  core::marker::Mark,
  impl_matcher,
};


use std::fmt::Debug;


pub trait Matcher<V>
where
  V: Debug + PartialEq + Sized + Sync
{
  fn new (value: V) -> Self;
  fn fail (&self, expected: &Matcherable<V>,  mark: Mark) -> String;
  fn matches (&self, expected: &Matcherable<V>) -> bool;
}


#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Matcherable<V>
where
  V: Debug + PartialEq + Sized + Sync
{
  pub value: V,
}


impl_matcher!(Matcherable);


#[macro_export]
macro_rules! impl_matcher {
  ($s:tt) => {
    impl<V> Matcher<V> for $s<V>
    where
      V: Debug + PartialEq + Sized + Sync
    {
      #[inline]
      fn new (value: V) -> Self {
        $s { value }
      }
    
      #[inline]
      fn matches (&self, value: &$s<V>) -> bool {
        *self == *value
      }
    
      #[inline]
      fn fail (&self, expected: &$s<V>, _mark: Mark) -> String {
        format!(
          r"received: <{:?}> expected: <{:?}>",
          self.value, expected.value,
        )
      }
    }
  };
}

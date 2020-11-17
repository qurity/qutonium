use crate::{
  core::marker::Mark,
  impl_matcher,
};


use std::fmt::Debug;


pub trait Matcher<V>
where
  V: Clone + Debug + PartialEq
{
  fn new (value: V) -> Self;
  fn fail (&self, expected: &Matcherable<V>,  mark: Mark) -> String;
  fn matches (&self, expected: &Matcherable<V>) -> bool;
}


#[derive(Eq, PartialEq)]
pub struct Matcherable<V>
where
  V: Clone + Debug + PartialEq
{
  pub value: V,
}


impl_matcher!(Matcherable);


#[macro_export]
macro_rules! impl_matcher {
  ($s:tt) => {
    impl<V> Clone for $s<V>
    where
      V: Clone + Debug + PartialEq
    {
      fn clone (&self) -> Self {
        $s {
          value: self.value.clone()
        }
      }
    }

    impl<V> Matcher<V> for $s<V>
    where
      V: Clone + Debug + PartialEq
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
          "received: {:?} expected: {:?}",
          self.value, expected.value,
        )
      }
    }
  };
}

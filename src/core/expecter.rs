use crate::core::{
  marker::Mark,
  matcher::{Matcher, Matcherable},
  sourcer::{Location, Source},
};


use std::fmt::Debug;


#[derive(Debug)]
pub struct Expect {
  pub diagnostic: Option<String>,
  pub location: Option<Location>,
}


#[derive(Clone, Copy)]
pub struct ExpectCase<V>
where
  V: Clone + Copy + Debug + PartialEq + Sized + Sync
{
  value: Matcherable<V>,
  source: Option<Source>,
}


impl<V> ExpectCase<V>
where
  V: Clone + Copy + Debug + PartialEq + Sized + Sync
{
  pub fn new (value: V) -> Self {
    ExpectCase {
      source: None,
      value: Matcherable::new(value),
    }
  }

  pub fn not (&mut self, matcher: Matcherable<V>) -> Result<Expect, Expect> {
    self.matching(self.value, matcher, Mark::Not)
  }

  pub fn to (&mut self, matcher: Matcherable<V>) -> Result<Expect, Expect> {
    self.matching(self.value, matcher, Mark::To)
  }

  pub fn source (mut self, source: Source) -> Self {
    self.source = Some(source);
    self
  }

  pub fn matching (
    self,
    received: Matcherable<V>,
    expected: Matcherable<V>,
    mark: Mark
  ) -> Result<Expect, Expect> {
    let success = if mark.is_assertion() {
      received.matches(&expected)
    } else {
      !received.matches(&expected)
    };

    match success {
      true => {
        let message = String::from("ok");

        let expect = Expect {
          diagnostic: Some(message),
          location: Some(self.source.unwrap().location),
        };

        Ok(expect)
      },
      _ => {
        let message = received.fail(&expected, mark);

        let expect = Expect {
          diagnostic: Some(message),
          location: Some(self.source.unwrap().location),
        };

        Err(expect)
      },
    }
  }
}


#[macro_export]
macro_rules! expect {
  (catch $body:block) => {{
    let location = Location::new(file!(), line!(), column!());
    let source = Source::new(location);
    let catched = std::panic::catch_unwind(|| { $body; });

    ExpectCase::new(catched.is_err()).source(source)
  }};
  ($value:expr) => {{
    let location = Location::new(file!(), line!(), column!());
    let source = Source::new(location);

    ExpectCase::new($value).source(source)
  }};
}


// TODO: $received type + location.
#[macro_export]
macro_rules! must {
  ($received:expr; eq $y:expr) => {{
    let location = Location::new(file!(), line!(), column!());
    let source = Source::new(location);

    Ok(expect!($received).source(source).to(be_equal($y))?)
  }};
  ($received:expr; ne $y:expr) => {{
    let location = Location::new(file!(), line!(), column!());
    let source = Source::new(location);

    Ok(expect!($received).source(source).not(be_equal($y))?)
  }};
  ($received:expr; be.falsy) => {{
    let location = Location::new(file!(), line!(), column!());
    let source = Source::new(location);

    Ok(expect!($received).source(source).to(be_false())?)
  }};
  ($received:expr; be.truthy) => {{
    let location = Location::new(file!(), line!(), column!());
    let source = Source::new(location);

    Ok(expect!($received).source(source).to(be_true())?)
  }};
  (catch $received:block to abort()) => {{
    let location = Location::new(file!(), line!(), column!());
    let source = Source::new(location);

    Ok(expect!(catch { $received }).source(source).to(abort())?)
  }};
}

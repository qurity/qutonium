#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;


  #[test]
  fn from_assertions () {
    suite!("the suite case name", {
      "test it should do something 1" || {
        fn square (x: u8) -> u8 { x * x }

        expect!(square(3)).to(be_equal(9))
      }
    });

    suite!("the suite case name", {
      "test it should do something 1" || {
        fn square (x: u8) -> u8 { x * x }

        must!(square(3); eq 9)
      }
    });
  }
}

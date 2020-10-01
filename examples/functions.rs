fn main () {}


#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;


  #[test]
  fn from_functions () {
    suite!("the suite case name", {
      "test it should do something 1" || {
        fn square (x) { x * x }
  
        expect!(square(3)).to(be_equal(9))
      }
    });
  }
}

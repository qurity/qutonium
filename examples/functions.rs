fn main () {}


#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;


  #[test]
  fn from_functions () {
    suite!("functions", {
      "calculate square 3" || {
        fn square (x: u8) -> u8 { x * x }
  
        expect!(square(3)).to(be_equal(9))
      }
    });
  }
}

fn main () {}


#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;


  #[test]
  fn from_functions () {
    suite!("suite 1", {
      "calculate square" || {
        fn square (x: u8) -> u8 { x * x }
  
        expect!(square(3)).to(be_equal(9))
      }
    });

    suite!("suite 2", {
      "add two numbers" || {
        fn sum (x: u8, y: u8) -> u8 { x + y }
  
        expect!(sum(2 + 4)).to(be_equal(6))
      }
    });
  }
}

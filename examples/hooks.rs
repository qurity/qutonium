fn main () {}


#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;


  #[test]
  fn from_hooks () {
    suite!("hooks", {
      before:all {
        println!("inside before:all!")
      }
  
      before:each {
        println!("inside before:each!")
      }
  
      after:all {
        println!("inside after:all!")
      }
  
      after:each {
        println!("inside after:each!")
      };

      "calculate square 3" || {
        fn square (x: u8) -> u8 { x * x }
  
        expect!(square(3)).to(be_equal(9))
      }

      "calculate square 7" || {
        fn square (x: u8) -> u8 { x * x }
  
        expect!(square(7)).not(be_equal(9))
      }
    });
  }
}

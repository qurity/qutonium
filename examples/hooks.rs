fn main () {}


#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;


  #[test]
  fn from_hooks () {
    suite!("describe suite test", {
      before:all {
        println!("before:all")
      }
  
      before:each {
        println!("before:each")
      }
  
      after:all {
        println!("after:all")
      }
  
      after:each {
        println!("after:each")
      };

      "test it should do something 1" || {
        fn square (x) { x * x }
  
        expect!(square(3)).to(be_equal(9))
      }

      "test it should do something 1" || {
        fn square (x) { x * x }
  
        expect!(square(3)).to(be_equal(9))
      }
    });
  }
}

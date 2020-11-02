fn main () {}


#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;


  #[test]
  fn from_must () {
    suite!("using must macro", {
      "compare boolean ~true" || {
        must!(true; be.truthy)
      }
  
      "compare boolean ~false" || {
        must!(false; be.falsy)
      }
  
      "calculate square ~be" || {
        fn square (x: u8) -> u8 { x * x }
  
        must!(square(3); be 9)
      }
  
      "calculate square ~ne" || {
        fn square (x: u8) -> u8 { x * x }
  
        must!(square(3); ne 9)
      }

      "testing panic" || {
        must!(catch { panic!() } to abort())
      }
    });
  }
}

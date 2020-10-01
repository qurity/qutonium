fn main () {}


#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;


  #[test]
  fn from_must () {
    suite!("the suite case name", {
      "test it should do something 3" || {
        must!(true; be.truthy)
      }
  
      "test it should do something 3" || {
        must!(false; be.falsy)
      }
  
      "calculate square" || {
        fn square (x) { x * x }
  
        must!(square(3); be 9)
      }
  
      "calculate square" || {
        fn square (x) { x * x }
  
        must!(square(3); ne 9)
      }

      "testing should panic" || {
        must!(catch { panic!() } to abort())
      }
    });
  }
}

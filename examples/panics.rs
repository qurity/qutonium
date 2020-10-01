fn main () {}


#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;


  #[test]
  fn from_panics () {
    suite!("the suite case name", {
      "should panic" || {
        expect!(catch { panic!() }).to(abort())
      }

      "should not panic" || {
        expect!(catch { println("no panic catched!") }).not(abort())
      }
    });
  }
}

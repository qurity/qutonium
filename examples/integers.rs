fn main () {}


#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;


  #[test]
  fn from_integers () {
    suite!("describe suite test", {
      "test it should do something 1" || {
        expect!(0).to(be_equal(0))
      }
    });
  }
}

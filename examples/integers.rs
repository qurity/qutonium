fn main () {}


#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;


  #[test]
  fn from_integers () {
    suite!("integers", {
      "compare values" || {
        expect!(0).to(be_equal(0))?;
        expect!(2 + 7).to(be_equal(9))?;
        expect!(0 == 0).to(be_true())
      }
    });
  }
}

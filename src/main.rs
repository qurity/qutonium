use qutonium::prelude::*;


fn main () {
  suite!("main: using expect macro", {
    "compare integer" || {
      expect!(2 + 2).to(be_equal(2))
    }
  });
}


#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;


  #[test]
  fn from_main () {
    suite!("main: using expect macro", {
      "compare integer" || {
        expect!(2 + 2).to(be_equal(4))
      }

      "compare boolean" || {
        expect!(false).to(be_false())?;
        expect!(true).to(be_true())
      }
  
      "testing should panic" || {
        expect!(catch { panic!() }).to(abort())
      }
    });
  }
}

use qutonium::prelude::*;


fn main () {
  suite!("the suite case name", {
    "compare integer" || {
      expect!(2 + 2).to(be_equal(2))
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

fn main () {}


#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;


  #[derive(Clone, Copy, PartialEq)]
  struct Can {
    flavor: &'static str,
    ounces: u8,
  }
  
  impl Can {
    pub fn new (flavor: &'static str, ounces: u8) -> Self {
      Can {
        flavor,
        ounces,
      }
    }
  }


  #[test]
  fn from_structs () {
    suite!("structs", {
      "compare two structs" || {
        // example from Jest
        let can1 = Can::new("grapefruit", 12);
        let can2 = Can::new("grapefruit", 12);

        expect!(can1).to(be_equal(can2))
      }
    })
  }
}

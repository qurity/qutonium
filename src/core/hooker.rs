use crate::utils::types;


use std::fmt::{Debug, Formatter, Result};


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HookType {
  AfterAll,
  AfterEach,
  BeforeAll,
  BeforeEach,
}


pub struct Hook {
  pub f: Box<dyn Fn() -> () + 'static>,
  pub kind: HookType,
  pub name: &'static str,
}


impl Debug for Hook {
  fn fmt (&self, f: &mut Formatter) -> Result {
    f.debug_struct("Hook")
    .field("f", &types::get_type_name_by_type(&self))
    .field("kind", &self.kind)
    .field("name", &self.name)
    .finish()
  }
}

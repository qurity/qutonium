use crate::utils::types;


use std::fmt::{Debug, Display, Formatter, Result};


pub struct Log<T> (T);


impl<T> Debug for Log<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{}", types::get_type_name_by_type(&self))
	}
}


impl<T> Display for Log<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{}", types::get_type_name_by_type(&self))
  }
}

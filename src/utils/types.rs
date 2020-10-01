use std::any;


pub fn get_type_name_by_type<T> (_: &T) -> String {
  any::type_name::<T>().into()
}

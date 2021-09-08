use crate::types::EString;

// not used yet
pub trait Core {
   fn eprint(string: EString) where Self: Sized;
}
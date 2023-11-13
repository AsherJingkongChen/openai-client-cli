use crate::{Entry, Result};

pub trait Loader<T>
where
  Self: Sized
{
  fn fetch(entry: &Entry) -> Result<Self>;
  fn value(self) -> T;
  fn value_ref(&self) -> &T;
}

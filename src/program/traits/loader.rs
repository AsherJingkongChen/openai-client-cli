use crate::{Entry, Result};

/// A trait to load an object from the program entry.
pub trait Loader<T>
where
  Self: Sized
{
  /// Fetch an object from the program entry.
  fn fetch(entry: &Entry) -> Result<Self>;
  /// Move the sealed object out.
  fn value(self) -> T;
  /// Returns the refernece of the sealed object.
  fn value_ref(&self) -> &T;
}

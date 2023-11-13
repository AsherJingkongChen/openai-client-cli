use crate::Result;
use std::path::Path;

/// A trait to create an object from a file.
pub trait FromFile
where
  Self: Sized
{
  /// Create an object from a file.
  fn from_file<P>(path: P) -> Result<Self>
  where
    P: AsRef<Path>;
}

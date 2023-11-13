use crate::Result;
use std::path::Path;

pub trait FromFile
where
  Self: Sized
{
  fn from_file<P>(path: P) -> Result<Self>
  where
    P: AsRef<Path>;
}

use crate::{Entry, Result, traits::*};
use std::{fs::File, path::Path, io::{stdout, Write}};
use tracing::{debug, info};

/// The output channel.
pub struct Output(Box<dyn Write>, bool);

impl Output {
  /// Check if the output channel is a file; otherwise, it is stdout.
  pub fn is_file(&self) -> bool {
    self.1
  }

  fn post_fetch_ok(self, target: &str) -> Result<Self> {
    info!("Successfully fetched the output channel into {target}");
    Ok(self)
  }
}

impl FromFile for Output {
  fn from_file<P>(path: P) -> Result<Self>
  where
    P: AsRef<Path>,
  {
    Ok(Self(Box::new(File::create(path)?), true))
  }
}

impl Loader<Box<dyn Write>> for Output {
  fn fetch(entry: &Entry) -> Result<Self> {
    if let Some(path) = entry.output_file.as_ref() {
      let target = &format!("the file {path:?}");
      match Output::from_file(path) {
        Ok(output) => return output.post_fetch_ok(target),
        Err(err) => debug!("Failed to create the output channel into {target}: {err:?}"),
      }
    }
    Self(Box::new(stdout()), false).post_fetch_ok("stdout")
  }
  fn value(self) -> Box<dyn Write> {
    self.0
  }
  fn value_ref(&self) -> &Box<dyn Write> {
    &self.0
  }
}

use crate::{Entry, Error, Result, traits::*};
use std::{fs::File, path::Path, io::{stdout, Write}};
use tracing::info;

pub struct Output(Box<dyn Write>);

impl FromFile for Output {
  fn from_file<P>(path: P) -> Result<Self>
  where
    P: AsRef<Path>,
  {
    Ok(Self(Box::new(File::create(path)?)))
  }
}

impl Loader<Box<dyn Write>> for Output {
  fn fetch(entry: &Entry) -> Result<Self> {
    let target_message =  entry.output_file
      .as_ref()
      .map_or(String::new(), |path| format!(" to the file {:?}", path));

    match entry.output_file
      .as_ref()
      .ok_or(Error::msg("Not provided"))
      .and_then(Output::from_file)
    {
      Ok(output) => {
        info!("Created the output channel{target_message}");
        return Ok(output);
      },
      Err(err) => info!(
        "Failed to create the output channel{target_message}: {err:?}"
      ),
    }

    info!("Piped the output channel to stdout");
    Ok(Self(Box::new(stdout())))
  }
  fn value(self) -> Box<dyn Write> {
    self.0
  }
  fn value_ref(&self) -> &Box<dyn Write> {
    &self.0
  }
}

impl<W> From<W> for Output
where
  W: 'static + Write,
{
  fn from(writer: W) -> Self {
    Self(Box::new(writer))
  }
}

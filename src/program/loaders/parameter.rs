use crate::{Entry, Error, Result, traits::*};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path, str::FromStr};
use tracing::{debug, info};

/// The API request parameters.
#[derive(Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Parameter(serde_json::Value);

impl Parameter {
  /// Create a new parameter object from bytes of JSON.
  pub fn from_slice(slice: &[u8]) -> Result<Self> {
    Ok(serde_json::from_slice(slice)?)
  }

  fn post_fetch_ok(self, source: &str) -> Result<Self> {
    info!(
      "Successfully fetched the API request parameters from {}: <JSON Object ({} bytes)>",
      source,
      self.value_ref().to_string().len(),
    );
    Ok(self)
  }
}

impl FromFile for Parameter {
  fn from_file<P>(path: P) -> Result<Self>
  where
    P: AsRef<Path>,
  {
    Ok(serde_json::from_reader(File::open(path)?)?)
  }
}

impl FromStr for Parameter {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self> {
    Ok(serde_json::from_str(text)?)
  }
}

impl Loader<serde_json::Value> for Parameter {
  fn fetch(entry: &Entry) -> Result<Self> {
    for path in [
        entry.parameter_file.as_ref(),
        Some(&"openai.json".into()),
        Some(&"openai-parameters.json".into()),
        Some(&"openai_parameters.json".into()),
        Some(&"openai-parameters".into()),
        Some(&"openai_parameters".into()),
        Some(&"openai.config.json".into()),
      ]
      .into_iter()
      .flatten()
    {
      let source = &format!("the file {path:?}");
      match Parameter::from_file(path) {
        Ok(parameter) => return parameter.post_fetch_ok(source),
        Err(err) => debug!("Failed to obtain the API request parameters from {source}: {err:?}"),
      }
    }
    Err(Error::msg("Failed to fetch the API request parameters"))
  }
  fn value(self) -> serde_json::Value {
    self.0
  }
  fn value_ref(&self) -> &serde_json::Value {
    &self.0
  }
}

impl From<serde_json::Value> for Parameter {
  fn from(value: serde_json::Value) -> Self {
    Self(value)
  }
}

impl From<&serde_json::Value> for Parameter {
  fn from(value: &serde_json::Value) -> Self {
    Self(value.clone())
  }
}

impl TryFrom<&str> for Parameter {
  type Error = Error;

  fn try_from(text: &str) -> Result<Self> {
    Self::from_str(text)
  }
}

impl TryFrom<String> for Parameter {
  type Error = Error;

  fn try_from(text: String) -> Result<Self> {
    Self::from_str(&text)
  }
}

impl TryFrom<&String> for Parameter {
  type Error = Error;

  fn try_from(text: &String) -> Result<Self> {
    Self::from_str(text)
  }
}

impl TryFrom<&[u8]> for Parameter {
  type Error = Error;

  fn try_from(text: &[u8]) -> Result<Self> {
    Self::from_slice(text)
  }
}

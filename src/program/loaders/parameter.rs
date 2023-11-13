use crate::{Entry, Error, Result, traits::*};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path, str::FromStr};
use tracing::info;

#[derive(Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Parameter(serde_json::Value);

impl Parameter {
  pub fn from_slice(slice: &[u8]) -> Result<Self> {
    Ok(serde_json::from_slice::<Parameter>(slice)?)
  }
}

impl FromFile for Parameter {
  fn from_file<P>(path: P) -> Result<Self>
  where
    P: AsRef<Path>,
  {
    Ok(serde_json::from_reader::<File, Parameter>(File::open(path)?)?)
  }
}

impl FromStr for Parameter {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self> {
    Ok(serde_json::from_str::<Parameter>(text)?)
  }
}

impl Loader<serde_json::Value> for Parameter {
  fn fetch(entry: &Entry) -> Result<Self> {
    let mut provided_files = Vec::new();
    if let Some(parameter_file) = entry.parameter_file.as_ref() {
      provided_files.push(parameter_file);
    }
    let default_files = [
      &"openai.json".into(),
      &"openai-parameters.json".into(),
      &"openai_parameters.json".into(),
      &"openai-parameters".into(),
      &"openai_parameters".into(),
      &"openai.config.json".into(),
    ];
    for file in provided_files.into_iter().chain(default_files.into_iter()) {
      match Parameter::from_file(file) {
        Ok(key) => return Ok(key),
        Err(err) => info!(
          "Failed to obtain the API request parameters from the file {file:?}: {err:?}"
        ),
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

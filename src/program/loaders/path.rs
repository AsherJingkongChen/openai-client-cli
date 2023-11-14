use crate::{Entry, Error, Result, traits::*};
use std::str::FromStr;
use regex::Regex;
use tracing::{debug, info};

/// The API request path.
pub struct Path(String);

impl FromStr for Path {
  type Err = Error;

  fn from_str(path: &str) -> Result<Self> {
    Ok(Self(
      Regex::new(include_str!("../../../assets/openai-openapi-paths-regex"))?
        .find(path)
        .ok_or(Error::msg("Invalid format of OpenAI API key"))?
        .as_str()
        .to_string()
    ))
  }
}

impl Loader<String> for Path {
  fn fetch(entry: &Entry) -> Result<Self> {
    let source = "the program arguments";
    match Path::from_str(&entry.path) {
      Ok(path) => {
        info!(
          "Successfully fetched the API request path from {source}: {:?}",
          path.value_ref(),
        );
        Ok(path)
      },
      Err(err) => {
        debug!("Failed to obtain the API request path from {source}: {err:?}");
        Err(Error::msg("Failed to fetch the API request path"))
      },
    }
  }
  fn value(self) -> String {
    self.0
  }
  fn value_ref(&self) -> &String {
    &self.0
  }
}

impl TryFrom<&str> for Path {
  type Error = Error;

  fn try_from(content: &str) -> Result<Self> {
    Self::from_str(content)
  }
}

impl TryFrom<String> for Path {
  type Error = Error;

  fn try_from(content: String) -> Result<Self> {
    Self::from_str(&content)
  }
}

impl TryFrom<&String> for Path {
  type Error = Error;

  fn try_from(content: &String) -> Result<Self> {
    Self::from_str(content)
  }
}

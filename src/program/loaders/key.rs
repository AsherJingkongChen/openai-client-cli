use crate::{Entry, Error, Result, traits::*};
use regex::Regex;
use shellexpand::path::tilde;
use std::{env, fs, path::{Path, PathBuf}, str::FromStr};
use tracing::{debug, info};

/// The API key.
pub struct Key(String);

impl Key {
  fn post_fetch_ok(self, source: &str) -> Result<Self> {
    info!(
      "Successfully fetched the API key from {source}: {:?}",
      format!("{}...{}", &self.value_ref()[..6], &self.value_ref()[49..]),
    );
    Ok(self)
  }
}

impl FromFile for Key {
  fn from_file<P>(path: P) -> Result<Self>
  where
    P: AsRef<Path>,
  {
    let text = fs::read_to_string(path)?;
    Self::from_str(&text)
  }
}

impl FromStr for Key {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self> {
    Ok(Self(
      Regex::new(r"sk-[[:alnum:]]{20}T3BlbkFJ[[:alnum:]]{20}")?
        .find(text)
        .ok_or(Error::msg("Invalid format of OpenAI API key"))?
        .as_str()
        .to_string()
    ))
  }
}

impl Loader<String> for Key {
  fn fetch(entry: &Entry) -> Result<Self> {
    if let Some(provided_file) = entry.key_file.as_ref() {
      let source = &format!("the provided file {provided_file:?}");
      match Key::from_file(provided_file) {
        Ok(key) => return key.post_fetch_ok(source),
        Err(err) => debug!("Failed to obtain the API key from {source}: {err:?}"),
      }
    }

    let source = "the environment variable `OPENAI_API_KEY`";
    match env::var("OPENAI_API_KEY")
      .map_err(Error::from)
      .and_then(Key::try_from)
    {
      Ok(key) => return key.post_fetch_ok(source),
      Err(err) => debug!("Failed to obtain the API key from {source}: {err:?}"),
    }

    for default_file in [
        &PathBuf::from("openai.env"),
        &PathBuf::from(".openai_profile"),
        &PathBuf::from(".env"),
        &PathBuf::from(tilde("~/openai.env")),
        &PathBuf::from(tilde("~/.openai_profile")),
        &PathBuf::from(tilde("~/.env")),
      ].into_iter()
    {
      let source = &format!("the default file {default_file:?}");
      match Key::from_file(default_file) {
        Ok(key) => return key.post_fetch_ok(source),
        Err(err) => debug!("Failed to obtain the API key from {source}: {err:?}"),
      }
    }
    Err(Error::msg("Failed to fetch the API key"))
  }
  fn value(self) -> String {
    self.0
  }
  fn value_ref(&self) -> &String {
    &self.0
  }
}

impl TryFrom<&str> for Key {
  type Error = Error;

  fn try_from(text: &str) -> Result<Self> {
    Self::from_str(text)
  }
}

impl TryFrom<String> for Key {
  type Error = Error;

  fn try_from(text: String) -> Result<Self> {
    Self::from_str(&text)
  }
}

impl TryFrom<&String> for Key {
  type Error = Error;

  fn try_from(text: &String) -> Result<Self> {
    Self::from_str(text)
  }
}

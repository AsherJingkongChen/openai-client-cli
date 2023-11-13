use crate::{Entry, Error, Result, traits::*};
use regex::Regex;
use shellexpand::path::tilde;
use std::{env, fs, path::{Path, PathBuf}, str::FromStr};
use tracing::info;

pub struct Key(String);

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
    let mut provided_files = Vec::new();
    if let Some(key_file) = entry.key_file.as_ref() {
      provided_files.push(key_file);
    }
    let default_files = [
      &PathBuf::from("openai.env"),
      &PathBuf::from(".openai_profile"),
      &PathBuf::from(".env"),
      &PathBuf::from(tilde("~/openai.env")),
      &PathBuf::from(tilde("~/.openai_profile")),
      &PathBuf::from(tilde("~/.env")),
    ];
    for file in provided_files.into_iter().chain(default_files.into_iter()) {
      match Key::from_file(file) {
        Ok(key) => return Ok(key),
        Err(err) => info!(
          "Failed to obtain the API key from the file {file:?}: {err:?}"
        ),
      }
    }
    match env::var("OPENAI_API_KEY")
      .map_err(Error::from)
      .and_then(Key::try_from)
    {
      Ok(key) => return Ok(key),
      Err(err) => info!(
        "Failed to obtain the API key from the variable `OPENAI_API_KEY`: {err:?}"
      ),
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

use crate::{Entry, Error, Result, traits::*};
use regex::Regex;
use shellexpand::path::tilde;
use std::{env, fs, path::{Path, PathBuf}, str::FromStr};
use tracing::info;

pub struct Organization {
  inner: String,
}

impl FromFile for Organization {
  fn from_file<P>(path: P) -> Result<Self>
  where
    P: AsRef<Path>,
  {
    let text = fs::read_to_string(path)?;
    Self::from_str(&text)
  }
}

impl FromStr for Organization {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self> {
    Ok(Self {
      inner: Regex::new(r"org-[[:alnum:]]{24}")?
        .find(text)
        .ok_or(Error::msg("Invalid format of OpenAI organization ID"))?
        .as_str()
        .to_string()
    })
  }
}

impl Loader<String> for Organization {
  fn fetch(entry: &Entry) -> Result<Self> {
    let mut provided_files = Vec::new();
    if let Some(organization_file) = entry.organization_file.as_ref() {
      provided_files.push(organization_file);
    }
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
      match Organization::from_file(file) {
        Ok(org) => return Ok(org),
        Err(err) => info!(
          "Failed to obtain the organization ID from the file {file:?}: {err:?}"
        ),
      }
    }
    match env::var("OPENAI_ORG_KEY")
      .map_err(Error::from)
      .and_then(Organization::try_from)
    {
      Ok(org) => return Ok(org),
      Err(err) => info!(
        "Failed to obtain the organization ID from the variable `OPENAI_ORG_KEY`: {err:?}"
      ),
    }
    Err(Error::msg("Failed to fetch theorganization ID"))
  }
  fn value(self) -> String {
    self.inner
  }
  fn value_ref(&self) -> &String {
    &self.inner
  }
}

impl TryFrom<&str> for Organization {
  type Error = Error;

  fn try_from(text: &str) -> Result<Self> {
    Self::from_str(text)
  }
}

impl TryFrom<String> for Organization {
  type Error = Error;

  fn try_from(text: String) -> Result<Self> {
    Self::from_str(&text)
  }
}

impl TryFrom<&String> for Organization {
  type Error = Error;

  fn try_from(text: &String) -> Result<Self> {
    Self::from_str(text)
  }
}

use crate::{Entry, Error, Result, traits::*};
use regex::Regex;
use shellexpand::path::tilde;
use std::{env, fs, path::{Path, PathBuf}, str::FromStr};
use tracing::{debug, info};

/// The organization ID.
pub struct Organization(String);

impl Organization {
  fn post_fetch_ok(self, source: &str) -> Result<Self> {
    info!(
      "Successfully fetched the organization ID from {source}: {:?}",
      format!("{}...{}", &self.value_ref()[..7], &self.value_ref()[26..]),
    );
    Ok(self)
  }
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
    Ok(Self(
      Regex::new(r"org-[[:alnum:]]{24}")?
        .find(text)
        .ok_or(Error::msg("Invalid format of OpenAI organization ID"))?
        .as_str()
        .to_string()
    ))
  }
}

impl Loader<String> for Organization {
  fn fetch(entry: &Entry) -> Result<Self> {
    for provided_file in [
        entry.organization_file.as_ref(),
        entry.key_file.as_ref(),
      ]
      .into_iter()
      .flatten()
    {
      let source = &format!("the provided file {provided_file:?}");
      match Organization::from_file(provided_file) {
        Ok(organization) => return organization.post_fetch_ok(source),
        Err(err) => debug!("Failed to obtain the organization ID from {source}: {err:?}"),
      }
    }

    let source = "the environment variable `OPENAI_ORG_KEY`";
    match env::var("OPENAI_ORG_KEY")
      .map_err(Error::from)
      .and_then(Organization::try_from)
    {
      Ok(organization) => return organization.post_fetch_ok(source),
      Err(err) => debug!("Failed to obtain the organization ID from {source}: {err:?}"),
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
      match Organization::from_file(default_file) {
        Ok(organization) => return organization.post_fetch_ok(source),
        Err(err) => debug!("Failed to obtain the organization ID from {source}: {err:?}"),
      }
    }
    Err(Error::msg("Failed to fetch the organization ID"))
  }
  fn value(self) -> String {
    self.0
  }
  fn value_ref(&self) -> &String {
    &self.0
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

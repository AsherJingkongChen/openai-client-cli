use crate::{Entry, Error, Result, traits::*};
use std::str::FromStr;
use tracing::info;

pub struct Method(http::Method);

impl Method {
  pub const CONNECT: Method = Method(http::Method::CONNECT);
  pub const DELETE: Method = Method(http::Method::DELETE);
  pub const GET: Method = Method(http::Method::GET);
  pub const HEAD: Method = Method(http::Method::HEAD);
  pub const OPTIONS: Method = Method(http::Method::OPTIONS);
  pub const PATCH: Method = Method(http::Method::PATCH);
  pub const POST: Method = Method(http::Method::POST);
  pub const PUT: Method = Method(http::Method::PUT);
  pub const TRACE: Method = Method(http::Method::TRACE);
}

impl FromStr for Method {
  type Err = Error;

  fn from_str(name: &str) -> Result<Self> {
    Ok(Self(http::Method::from_str(name)?))
  }
}

impl Loader<http::Method> for Method {
  fn fetch(entry: &Entry) -> Result<Self> {
    match entry.method
      .as_ref()
      .ok_or(Error::msg("Not provided"))
      .and_then(Method::try_from)
    {
      Ok(method) => return Ok(method),
      Err(err) => info!(
        "Failed to obtain the API request method from command line arguments: {err:?}"
      ),
    }
    if entry.parameter.is_some() {
      info!("The API request parameters were fetched successfully");
      Ok(Method::POST)
    } else {
      info!("The API request parameters were not fetched successfully");
      Ok(Method::GET)
    }
  }
  fn value(self) -> http::Method {
    self.0
  }
  fn value_ref(&self) -> &http::Method {
    &self.0
  }
}

impl From<http::Method> for Method {
  fn from(method: http::Method) -> Self {
    Self(method)
  }
}

impl From<&http::Method> for Method {
  fn from(method: &http::Method) -> Self {
    Self(method.clone())
  }
}

impl TryFrom<&str> for Method {
  type Error = Error;

  fn try_from(text: &str) -> Result<Self> {
    Self::from_str(text)
  }
}

impl TryFrom<String> for Method {
  type Error = Error;

  fn try_from(text: String) -> Result<Self> {
    Self::from_str(&text)
  }
}

impl TryFrom<&String> for Method {
  type Error = Error;

  fn try_from(text: &String) -> Result<Self> {
    Self::from_str(text)
  }
}

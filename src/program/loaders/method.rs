use crate::{Entry, Error, Result, traits::*};
use std::str::FromStr;
use tracing::{debug, info};

/// The HTTP method for the API request.
pub struct Method(http::Method);

impl Method {
  /// CONNECT method.
  pub const CONNECT: Method = Method(http::Method::CONNECT);
  /// DELETE method.
  pub const DELETE: Method = Method(http::Method::DELETE);
  /// GET method.
  /// 
  /// If the API request parameters (body) are not provided, this method is preferred.
  pub const GET: Method = Method(http::Method::GET);
  /// HEAD method.
  pub const HEAD: Method = Method(http::Method::HEAD);
  /// OPTIONS method.
  pub const OPTIONS: Method = Method(http::Method::OPTIONS);
  /// PATCH method.
  pub const PATCH: Method = Method(http::Method::PATCH);
  /// POST method.
  pub const POST: Method = Method(http::Method::POST);
  /// PUT method.
  pub const PUT: Method = Method(http::Method::PUT);
  /// TRACE method.
  pub const TRACE: Method = Method(http::Method::TRACE);

  fn post_fetch_ok(self, source: &str) -> Result<Self> {
    info!(
      "Successfully fetched the API request method from {}: {:?}",
      source, self.value_ref(),
    );
    Ok(self)
  }
}

impl FromStr for Method {
  type Err = Error;

  fn from_str(name: &str) -> Result<Self> {
    Ok(Self(http::Method::from_str(name)?))
  }
}

impl Loader<http::Method> for Method {
  fn fetch(entry: &Entry) -> Result<Self> {
    let source_ok = "the program arguments";
    match entry.method
      .as_ref()
      .ok_or(Error::msg("Not provided"))
      .and_then(Method::try_from)
    {
      Ok(method) => method.post_fetch_ok(source_ok),
      Err(err) => {
        debug!("Failed to obtain the API request method from {source_ok}: {err:?}");
        let (method, dep_status) = if entry._parameter.is_some() {
          (Self::POST, "")
        } else {
          (Self::GET, "un")
        };
        debug!("The API request parameters were fetched {dep_status}successfully");
        method.post_fetch_ok("the fallback options")
      },
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

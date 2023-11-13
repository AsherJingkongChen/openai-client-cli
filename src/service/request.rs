use crate::{loaders::*, Result, traits::*};
use url::Url;

/// The request object.
pub struct OpenAIRequest {
  /// The HTTP method.
  pub method: Method,

  /// The parameter object (request body).
  pub parameter: Option<Parameter>,

  /// The URL.
  pub url: Url,
}

impl OpenAIRequest {
  /// Create a new request object.
  pub fn new(
    method: Method,
    path: Path,
    parameter: Option<Parameter>,
  ) -> Result<Self> {
    Ok(Self {
      method,
      parameter,
      url: Url::parse("https://api.openai.com/v1/")?.join(&path.value())?,
    })
  }
}

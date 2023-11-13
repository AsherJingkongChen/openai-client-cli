use crate::{loaders::*, Result, traits::*};
use url::Url;

pub struct OpenAIRequest {
  pub method: Method,
  pub parameter: Option<Parameter>,
  pub url: Url,
}

impl OpenAIRequest {
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

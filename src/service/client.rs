use crate::{loaders::*, service::*, traits::*, Result};
use http::header::AUTHORIZATION;
use reqwest::{
  Client as ReqwestClient,
  Response,
};
use tracing::info;

pub struct OpenAIClient {
  pub key: Key,
  pub organization: Option<Organization>,
}

impl OpenAIClient {
  pub fn new(key: Key, organization: Option<Organization>) -> Self {
    Self { key, organization }
  }
  pub async fn send(&self, request: OpenAIRequest) -> Result<Response> {
    let method = request.method.value();
    let url = request.url;
    let authorization = format!("Bearer {}", self.key.value_ref());
    let organization = self.organization
      .as_ref()
      .map(Loader::value_ref);
    let body = request.parameter
      .as_ref()
      .map(Loader::value_ref);

    let client = ReqwestClient::new();

    info!("Sending request to {}", url);
    let mut request = client.request(method, url);
    request = request.header(AUTHORIZATION, authorization);
    if let Some(organization) = organization {
      request = request.header("OpenAI-Organization", organization);
    }
    if let Some(body) = body {
      request = request.json(body);
    }
    let response = request.send().await?;
    info!("Received the API response: {:#?}", response.status());
    Ok(response)
  }
}

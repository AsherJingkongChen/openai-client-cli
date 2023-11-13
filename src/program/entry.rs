use crate::*;
use clap::{arg, command};
use eventsource_stream::Eventsource;
use futures_util::StreamExt;
use http::header::CONTENT_TYPE;
use std::path::PathBuf;
use std::io::stderr;
use tracing::{debug, info, Level};
pub use clap::Parser;

#[derive(Parser)]
#[command(
  about,
  author,
  bin_name = "openai",
  help_template = "\
{before-help}\
{name} {version} by {author}
{about}

{usage-heading} {usage}

{all-args}\
{after-help}",
  name = "openai",
  version,
  next_line_help = true,
)]
pub struct Entry {
  #[arg(
    help = "\
The file path where the API key is stored.
The program will attempt the following steps to obtain a valid API key:
 1. Read the file from the provided path <KEY_FILE_PATH>.
 2. Read the file from the default paths in the following order:
    `openai.env`, `.openai_profile`, `.env`,
    `~/openai.env`, `~/.openai_profile` or `~/.env`.
 3. Read the environment variable `OPENAI_API_KEY`.
 4. Exit the program with a non-zero return code.
",
    long,
    short = 'k',
    value_name = "KEY_FILE_PATH",
  )]
  pub key_file: Option<PathBuf>,

  #[arg(
    help = "\
The HTTP method used for the API request.
The program will attempt the following steps to determine a valid HTTP method:
 1. Read the argument value <METHOD>.
 2. If the `parameter` object is fetched successfully from either
    <PARAM_FILE_PATH> or one of the default paths, set <METHOD> to `POST`.
 3. Otherwise, set <METHOD> to `GET`.
",
    long,
    short = 'm',
    value_name = "METHOD",
  )]
  pub method: Option<String>,

  #[arg(
    help = "\
The file path where the organization ID is stored.
The program will attempt the following steps to obtain a valid organization ID:
 1. Read the file from the provided path <ORG_FILE_PATH>.
 2. Read the file from provided path of key file <KEY_FILE_PATH>.
 3. Read the file from the default paths in the following order:
    `openai.env`, `.openai_profile`, `.env`,
    `~/openai.env`, `~/.openai_profile` or `~/.env`.
 4. Read the environment variable `OPENAI_ORG_KEY`.
 5. Ignore the field and leave it empty.
",
    short = 'g',
    long = "org-file",
    value_name = "ORG_FILE_PATH",
  )]
  pub organization_file: Option<PathBuf>,

  #[arg(
    help = "\
The file path where the API response will be stored.
The program will attempt the following steps to successfully store the response:
 1. Export the output to the provided file path <OUTPUT_FILE_PATH>.
 2. Export the output to stdout (to the terminal or piped to another program).
 3. Exit the program with a non-zero return code.
",
    long,
    short = 'o',
    value_name = "OUTPUT_FILE_PATH",
  )]
  pub output_file: Option<PathBuf>,

  #[arg(
    help = "\
The file path where the API request parameters (body) are stored in JSON format.
The program will attempt the following steps to obtain a valid parameter object:
 1. Read the file from the provided path <PARAM_FILE_PATH>.
 2. Read the file from the default paths in the following order:
    `openai.json`, `openai-parameters.json`, `openai_parameters.json`,
    `openai-parameters`, `openai_parameters`, or `openai.config.json`.
 3. Ignore the field and leave it empty
",
    long,
    short = 'p',
    value_name = "PARAM_FILE_PATH",
  )]
  pub parameter_file: Option<PathBuf>,

  #[arg(hide = true, long, exclusive = true)]
  pub parameter: Option<Parameter>,

  #[arg(
    help = "\
The API request path. (part of the URL)
The program will use regex to extract the matched segment in <PATH>.
For example, the extracted strings will be the same when <PATH> is either
`chat/completions`, `/chat/completions` or `https://api.openai.com/v1/chat/completions`.",
    value_name = "PATH",
  )]
  pub path: String,

  #[arg(
    default_value = "false",
    help = "\
Enable verbose logging mode. This mode is useful for debugging purposes.
It is disabled by default.
",
    long,
    short = 'v',
  )]
  pub verbose: bool,
}

impl Entry {
  pub async fn run(mut self) -> Result<()> {
    let logger = tracing_subscriber::fmt()
      .with_target(false)
      .with_writer(stderr)
      .without_time();
    if self.verbose {
      logger
        .with_max_level(Level::DEBUG)
        .with_file(true)
        .with_line_number(true)
        .init();
    } else {
      logger
        .with_max_level(Level::WARN)
        .init();
    }

    let key = Key::fetch(&self)?;
    info!(
      "Fetched the API key: {:?}",
      format!("{}...{}", &key.value_ref()[..6], &key.value_ref()[49..]),
    );

    let organization = Organization::fetch(&self).ok();
    match &organization {
      Some(organization) => info!(
        "Fetched the organization ID: {:?}",
        format!("{}...{}", &organization.value_ref()[..7], &organization.value_ref()[26..]),
      ),
      None => info!("Ignored the field `organization` for not being fetched successfully"),
    }

    let output = Output::fetch(&self)?;
    info!("Fetched the output channel");

    let path = Path::fetch(&self)?;
    info!("Fetched the API request path: {:?}", path.value_ref());

    // `parameter` should be fetched before `method`
    let parameter = Parameter::fetch(&self).ok();
    match &parameter {
      Some(parameter) => info!(
        "Fetched the API request parameters: <JSON Object ({} bytes)>",
        serde_json::to_vec(&parameter)?.len(),
      ),
      None => info!("Ignored the field `parameter` for not being fetched successfully"),
    }
    self.parameter = parameter;

    let method = Method::fetch(&self)?;
    info!("Fetched the API request method: {:?}", method.value_ref());

    let client = OpenAIClient::new(key, organization);
    let request = OpenAIRequest::new(method, path, self.parameter)?;
    let response = client.send(request).await?;
    debug!("\n{:#?}", response);

    let status_error = response.error_for_status_ref().map(|_| ());
    let content_type = response
      .headers()
      .get(CONTENT_TYPE)
      .ok_or(Error::msg("The API response does not contain the header `Content-Type`"))?;
    info!("Resolve the API response content type: {:?}", content_type.to_str());

    match content_type.as_bytes() {
      b"application/json" => {
        let response_json = response
          .json::<serde_json::Value>()
          .await
          .map_err(Error::from)
          .and_then(|object| {
            serde_json::to_string_pretty(&object)
              .map_err(Error::from)
          });
        if let Ok(response_json) = &response_json {
          info!("Resolved the API response: <JSON Object ({} bytes)>", response_json.len());
        }

        if response_json.is_err() || status_error.is_err() {
          Err(Error::msg("\x1b[F")
              .context(response_json.map_or_else(
                |e| e.to_string(),
                |json| format!("The API response in JSON format:\n{}", json)),
              )
              .context(status_error.map_or_else(
                |e| e.to_string(),
                |_| String::new(),
              ))
              .context("Failed to resolve the API response")
          )
        } else {
          let response_json = response_json.unwrap();
          info!("Exporting the output");
          output.value().write_all(response_json.as_bytes())?;
          Ok(())
        }
      },
      b"text/event-stream" => {
        status_error?; // should not be an error

        info!("Resolving the API response and exporting the output");

        let mut stream = response.bytes_stream().eventsource();
        let mut output = output.value();
        while let Some(chunk) = stream.next().await {
          let chunk = chunk?;
          let data = chunk.data;
          info!(
            "Resolved the API response: <Event Stream Data: ({} bytes)>",
            data.len(),
          );
          if data == "[DONE]" {
            info!("Read the end of the API response: {}", data);
            break;
          }
          output.write_all(&[data.as_bytes(), b"\n"].concat())?;
        }
        Ok(())
      },
      _ => Err(Error::msg("Failed to resolve API response: Invalid format")),
    }
  }
}

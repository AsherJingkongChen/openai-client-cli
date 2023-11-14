use crate::*;
use clap::{arg, command};
use eventsource_stream::Eventsource;
use futures_util::StreamExt;
use http::header::CONTENT_TYPE;
use std::path::PathBuf;
use std::io::stderr;
use tracing::{debug, info, Level};

#[doc(hidden)]
pub use clap::Parser;

/// The main entry-point for the program.
#[derive(Parser)]
#[command(
  author,
  about,
  bin_name = "openai-client",
  help_template = "\
{before-help}\
{name} {version} by {author}
{about}

{usage-heading} {usage}

{all-args}\
{after-help}",
  version,
  next_line_help = true,
)]
pub struct Entry {
  /// The file path where the API key is stored.
  #[arg(
    help = "\
The file path where the API key is stored.
The program will attempt the following steps to obtain a valid API key:
 1. Read the file from the provided path <KEY_FILE_PATH>.
 2. Read the environment variable `OPENAI_API_KEY`.
 3. Read the file from the default paths in the following order:
    `openai.env`, `.openai_profile`, `.env`,
    `~/openai.env`, `~/.openai_profile` or `~/.env`.
 4. Exit the program with a non-zero return code.
",
    long,
    short = 'k',
    value_name = "KEY_FILE_PATH",
  )]
  pub key_file: Option<PathBuf>,

  /// The HTTP method used for the API request.
  #[arg(
    help = "\
The HTTP method used for the API request.
The program will attempt the following steps to determine a valid HTTP method:
 1. Read the argument <METHOD>.
 2. If the `parameter` object is successfully fetched from either
    <PARAM_FILE_PATH> or one of the default paths, set <METHOD> to `POST`.
 3. Otherwise, set <METHOD> to `GET`.
",
    long,
    short = 'm',
    value_name = "METHOD",
  )]
  pub method: Option<String>,

  /// The file path where the organization ID is stored.
  #[arg(
    help = "\
The file path where the organization ID is stored.
The program will attempt the following steps to obtain a valid organization ID:
 1. Read the file from the provided path <ORG_FILE_PATH>.
 2. Read the file from provided path of key file <KEY_FILE_PATH>.
 3. Read the environment variable `OPENAI_ORG_KEY`.
 4. Read the file from the default paths in the following order:
    `openai.env`, `.openai_profile`, `.env`,
    `~/openai.env`, `~/.openai_profile` or `~/.env`.
 5. Ignore the field and leave it empty.
",
    short = 'g',
    long = "org-file",
    value_name = "ORG_FILE_PATH",
  )]
  pub organization_file: Option<PathBuf>,

  /// The file path where the API response will be stored.
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

  /// The file path where the API request parameters (body) are stored in JSON format.
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

  /// Hidden.
  #[arg(hide = true, long, exclusive = true)]
  pub _parameter: Option<Parameter>,

  /// The API request path. (part of the URL)
  #[arg(
    help = "\
The API request path. (part of the URL)
The program will use regex to extract the matched segment in <PATH>.
For example, the extracted strings will be the same when <PATH> is either
`chat/completions`, `/chat/completions` or `https://api.openai.com/v1/chat/completions`.",
    value_name = "PATH",
  )]
  pub path: String,

  /// Switch for verbose logging mode.
  #[arg(
    default_value = "false",
    help = "\
Switch for verbose logging mode. This mode is useful for debugging purposes.
It is disabled by default.
",
    long,
    short = 'v',
  )]
  pub verbose: bool,
}

impl Entry {
  /// Run the program.
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
    let organization = Organization::fetch(&self).ok();
    if organization.is_none() {
      info!("Ignored the field `organization` for not being fetched successfully");
    }
    let output = Output::fetch(&self)?;
    // `parameter` should be fetched before `method`
    let parameter = Parameter::fetch(&self).ok();
    if parameter.is_none() {
      info!("Ignored the field `parameter` for not being fetched successfully");
    }
    self._parameter = parameter;
    let path = Path::fetch(&self)?;
    let method = Method::fetch(&self)?;

    let client = OpenAIClient::new(key, organization);
    let request = OpenAIRequest::new(method, path, self._parameter)?;
    let response = client.send(request).await?;
    debug!("\n{:#?}", response);

    let status_error = response.error_for_status_ref().map(|_| ());
    let content_type = response
      .headers()
      .get(CONTENT_TYPE)
      .ok_or(Error::msg("The API response does not contain the header `Content-Type`"))?
      .to_str()
      .unwrap_or("unknown");
    info!(
      "Resolving the API response in the content type: {:?}",
      content_type,
    );

    let output_target = if output.is_file() { "the file" } else { "stdout" };

    match content_type {
      "application/json" => {
        let response_json = response
          .json::<serde_json::Value>()
          .await
          .map_err(Error::from)
          .and_then(|object| {
            serde_json::to_string_pretty(&object)
              .map_err(Error::from)
          });
        if let Ok(response_json) = &response_json {
          info!(
            "Resolved the API response: <JSON Object ({} bytes)>",
            response_json.len(),
          );
        }

        if response_json.is_err() || status_error.is_err() {
          Err(
            Error::msg("\u{1b}[F")
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
          let mut output = output.value();
          info!("Exporting the output to {output_target}");
          output.write_all(response_json.as_bytes())?;
          Ok(())
        }
      },
      "text/event-stream" => {
        status_error?; // should not be an error

        info!("Exporting the output to {output_target}");
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
            info!("Reached the end of the API response");
            break;
          }
          if chunk.retry.is_some() {
            return Err(Error::msg("Failed to resolve API response: Retry occurred"));
          }
          output.write_all(&[data.as_bytes(), b"\n"].concat())?;
        }
        Ok(())
      },
      unknown_type => Err(Error::msg(format!(
        "Failed to resolve API response: {unknown_type:?} is an invalid format"
      ))),
    }
  }
}

use openai_client_cli::{entry::*, loaders::*, traits::*};
use std::{env, io::stderr};
use anyhow::Result;

#[test]
fn fetch() -> Result<()> {
  tracing_subscriber::fmt()
    .without_time()
    .with_file(true)
    .with_line_number(true)
    .with_max_level(tracing::Level::DEBUG)
    .with_target(false)
    .with_writer(stderr)
    .init();

  env::set_current_dir("tests")?;
  env::set_var("OPENAI_ORG_KEY", "org-12345678901234567Correct");

  let command = "openai /v1/models -v";
  let mut entry = Entry::parse_from(command.split(' '));
  let key = Key::fetch(&entry)?;
  let organization = Organization::fetch(&entry);
  let output = Output::fetch(&entry)?;
  let parameter = Parameter::fetch(&entry); // `parameter` should be fetched before `method`
  entry._parameter = parameter.ok();
  let path = Path::fetch(&entry)?;
  let method = Method::fetch(&entry)?;
  
  assert_eq!(organization.is_ok(), true);
  assert_eq!(entry._parameter.is_some(), true);

  assert_eq!(key.value_ref(), "sk-abcdeABCDE1234567890T3BlbkFJ1234567890abcdeABCDE");
  assert_eq!(method.value_ref(), "POST");
  assert_eq!(organization.unwrap().value_ref(), "org-12345678901234567Correct");
  assert_eq!(output.is_file(), false);
  assert_eq!(path.value_ref(), "models");

  Ok(())
}
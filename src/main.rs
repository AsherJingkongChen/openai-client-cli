use openai_cli::entry::*;
use std::{process::exit, env::args_os};
use tracing::error;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  if let Err(err) = Entry::parse_from(args_os()).run().await {
    error!("{:?}", err);
    exit(1);
  }
}

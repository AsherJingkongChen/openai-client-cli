use anyhow::Result;

mod openai_openapi {
  use crate::Result;
  use regex::Regex;
  use std::{fs::{File, self}, path::Path};
  use serde::Deserialize;

  #[derive(Deserialize)]
  struct OpenAIManifest {
    paths: serde_yaml::Mapping,
  }

  pub fn build() -> Result<()>
  {
    let manifest_file = "external/openai-openapi/openapi.yaml";
    let asset_file = "assets/openai-openapi-paths-regex";
    println!("cargo:rerun-if-changed={manifest_file}");

    let paths = fetch_paths(manifest_file)?;
    let regex = convert_paths_to_regex(&paths)?;
    fs::write(asset_file, regex)?;
    Ok(())
  }

  fn fetch_paths<P>(path: P) -> Result<Vec<String>>
  where
    P: AsRef<Path>,
  {
    Ok(serde_yaml::from_reader::<File, OpenAIManifest>(File::open(path)?)?
      .paths
      .keys()
      .filter_map(serde_yaml::Value::as_str)
      .map(|s| s[1..].to_string())
      .collect()
    )
  }

  fn convert_paths_to_regex(paths: &[String]) -> Result<String> {
    let padded_str = format!("(({}$))", paths.join("$)|("));
    let fslash_escaped_str = Regex::new(r"/")?
      .replace_all(&padded_str, r"\/");
    let wildcard_transformed_str = Regex::new(r"\{.*?\}")?
      .replace_all(&fslash_escaped_str, r".*");
    Ok(wildcard_transformed_str.to_string())
  }
}

fn main() -> Result<()> {
  openai_openapi::build()?;
  Ok(())
}

#![doc = include_str!("../README.md")]
#![doc = include_str!("../docs/manual-help.md")]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// The CLI program.
pub mod program;
pub use program::*;

/// The OpenAI API service.
pub mod service;
pub use service::*;

#[doc(hidden)]
pub use anyhow::{Error, Result};

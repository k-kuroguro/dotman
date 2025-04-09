use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
   #[error("Failed to load config file: {0}")]
   FailedToLoadConfig(#[from] std::io::Error),
   #[error("Failed to parse config file: {0}")]
   FailedToParseConfig(#[from] serde_yaml::Error),
   #[error("Failed to expand tilde in path: {0}")]
   FailedToExpandTilde(PathBuf),
   #[error("{0}")]
   Other(#[from] Box<dyn std::error::Error>),
}

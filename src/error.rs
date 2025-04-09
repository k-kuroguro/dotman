use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
   #[error(
      "Dotfiles directory is not set\nPlease use --dotfiles-dir option or set DOTFILES_DIR environment variable"
   )]
   UndefinedDotfilesDir,
   #[error("Failed to load config file: {0}")]
   FailedToLoadConfig(#[from] std::io::Error),
   #[error("Failed to parse config file: {0}")]
   FailedToParseConfig(#[from] serde_yaml::Error),
   #[error("{0}")]
   Other(#[from] Box<dyn std::error::Error>),
}

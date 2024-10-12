use thiserror::Error;

/// The default error type for `xenon`
#[derive(Error, Debug, Default)]
pub enum Error {
  #[default] #[error("Unknown Error")] Unknown,
  #[error(transparent)] Generic(#[from] Box<dyn std::error::Error>),
  #[error("IO Error: {0}")] IO(#[from] std::io::Error)
}

/// The default result type for `xenon`
pub type Result<T> = std::result::Result<T, Error>;
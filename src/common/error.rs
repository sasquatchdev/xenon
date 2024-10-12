use thiserror::Error;

#[derive(Error, Debug, Default)]
pub enum Error {
  #[default] #[error("Unknown Error")] Unknown,
  #[error(transparent)] Generic(#[from] Box<dyn std::error::Error>),
  #[error("IO Error: {0}")] IO(#[from] std::io::Error)
}

pub type Result<T> = std::result::Result<T, Error>;
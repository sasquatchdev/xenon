use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown Error: {0}")]
    Unknown(&'static str),

    #[error("IO Error: {0}")]
    InOut(std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

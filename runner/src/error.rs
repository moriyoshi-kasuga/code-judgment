#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed io operation: {0}")]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

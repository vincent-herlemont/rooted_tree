mod node;
mod dag;


use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Dag error: {0}")]
    Dag(String),
}

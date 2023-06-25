mod id;
mod implementation;
mod node;
mod rtree;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Dag error: {0}")]
    Dag(String),
    #[error("Root node already exists")]
    RootNodeAlreadyExists,
    #[error("Parent node does not exist")]
    ParentNodeDoesNotExist,
}

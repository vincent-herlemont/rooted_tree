mod diff;
mod from;
mod node;
mod rooted_tree;

#[cfg(test)]
mod test_data;

pub use crate::node::Node;
pub use crate::rooted_tree::RootedTree;

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
    #[error("Node does not exist")]
    NodeDoesNotExist,
}

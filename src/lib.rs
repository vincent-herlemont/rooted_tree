mod diff;
mod node;
mod report;
mod rooted_tree;
mod try_from;

#[cfg(test)]
mod test_data;

pub use crate::node::Node;
pub use crate::report::*;
pub use crate::rooted_tree::RootedTree;
pub use crate::try_from::*;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Root node already exists")]
    RootNodeAlreadyExists,
    #[error("Parent node does not exist")]
    ParentNodeDoesNotExist,
    #[error("Node does not exist")]
    NodeDoesNotExist,
    #[error("Parent node does not contain child")]
    ParentNodeDoesNotContainChild,
    #[error("Child node has no parent")]
    ChildNodeHasNoParent,
    #[error("Root node has parent")]
    RootNodeHasParent,
    #[error("Report error")]
    ReportError(#[from] report::Error),
}

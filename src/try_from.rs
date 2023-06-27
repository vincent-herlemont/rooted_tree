use crate::{Node, Result, RootedTree};
use std::hash::Hash;

impl<I: Eq + PartialEq + Clone + Hash, N: Node<I>> TryFrom<Vec<N>> for RootedTree<I, N> {
    type Error = crate::Error;

    fn try_from(vec: Vec<N>) -> Result<Self> {
        let mut rooted_tree = RootedTree::new();
        let mut iter = vec.into_iter();
        if let Some(root_node) = iter.next() {
            rooted_tree.set_root_node(root_node);
        }

        for node in iter {
            rooted_tree.set_child_node(node)?;
        }

        Ok(rooted_tree)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_data::DataNode;
    use crate::*;

    #[test]
    fn from_vec() {
        let mut list_node = vec![];
        let mut node = DataNode::new(1);
        node.add_child_id(2);
        list_node.push(node);

        let mut node = DataNode::new(2);
        node.set_parent_id(1);
        list_node.push(node);

        let tree: RootedTree<i32, DataNode> = list_node.try_into().unwrap();
        assert_eq!(tree.len(), 2);
    }
}

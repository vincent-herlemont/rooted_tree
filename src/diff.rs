use crate::{Node, RootedTree};
use std::hash::Hash;

impl<I: Eq + PartialEq + Clone + Hash, N: Node<I>> RootedTree<I, N> {
    pub fn diff(&self, rooted_tree: &RootedTree<I, N>) -> RootedTree<I, N> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::*;

    #[test]
    fn test_diff() {
        let mut tree1 = RootedTree::<i32, DataNode>::new();
        let node1 = DataNode::new(1);
        let node2 = DataNode::new(2);

        let mut tree2 = RootedTree::<i32, DataNode>::new();
        let node3 = DataNode::new(1);
        let node4 = DataNode::new(3);

        let mut expected_tree = RootedTree::<i32, DataNode>::new();
        let node5 = DataNode::new(2);
    }
}

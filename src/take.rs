use crate::{Node, RootedTree};
use std::hash::Hash;

impl<I: Eq + PartialEq + Clone + Hash, N: Node<I>> RootedTree<I, N> {
    pub fn take(&mut self, id: I) -> Option<RootedTree<I, N>> {
        // Take the root node
        if let Some(root_node) = &self.root_node {
            if root_node.id() == id {
                let mut sub_tree = RootedTree::new();
                sub_tree.root_node = self.root_node.take();
                for (child_id, node) in self.child_nodes.drain() {
                    sub_tree.child_nodes.insert(child_id, node);
                }
                return Some(sub_tree);
            }
        }

        // Take from a middle node
        let children = self.list_all_child_ids(&id);
        let mut sub_tree = RootedTree::new();
        sub_tree.root_node = self.child_nodes.remove(&id);
        for child_id in children {
            if let Some(node) = self.child_nodes.remove(&child_id) {
                sub_tree.child_nodes.insert(child_id.clone(), node);
            }
        }

        Some(sub_tree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::*;

    #[test]
    fn take_root() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();

        let sub_tree = tree.take(1).unwrap();

        assert_eq!(tree.len(), 0);

        assert_eq!(sub_tree.len(), 2);
        assert_eq!(sub_tree.get_node(&1).unwrap().parent_id(), None);
        assert_eq!(sub_tree.get_node(&1).unwrap().child_ids_vec(), vec![2]);
    }

    #[test]
    fn take_child() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(2), DataNode::new(3)).unwrap();

        let sub_tree = tree.take(2).unwrap();

        assert_eq!(tree.len(), 1);
        assert_eq!(tree.get_node(&1).unwrap().parent_id(), None);
        assert_eq!(tree.get_node(&1).unwrap().child_ids_vec(), vec![2]);

        assert_eq!(sub_tree.len(), 2);
        assert_eq!(sub_tree.get_node(&2).unwrap().parent_id(), Some(1));
        assert_eq!(sub_tree.get_node(&2).unwrap().child_ids_vec(), vec![3]);
        assert_eq!(sub_tree.get_node(&3).unwrap().parent_id(), Some(2));
        assert_eq!(sub_tree.get_node(&3).unwrap().child_ids_vec(), vec![]);
    }

    #[test]
    fn take_end_child() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(2), DataNode::new(3)).unwrap();

        let sub_tree = tree.take(3).unwrap();

        assert_eq!(tree.len(), 2);
        assert_eq!(tree.get_node(&1).unwrap().parent_id(), None);
        assert_eq!(tree.get_node(&1).unwrap().child_ids_vec(), vec![2]);
        assert_eq!(tree.get_node(&2).unwrap().parent_id(), Some(1));
        assert_eq!(tree.get_node(&2).unwrap().child_ids_vec(), vec![3]);

        assert_eq!(sub_tree.len(), 1);
        assert_eq!(sub_tree.get_node(&3).unwrap().parent_id(), Some(2));
        assert_eq!(sub_tree.get_node(&3).unwrap().child_ids_vec(), vec![]);
    }
}

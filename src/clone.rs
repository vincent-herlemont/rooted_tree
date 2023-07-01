use crate::{Node, RootedTree};
use std::hash::Hash;

impl<I: Eq + PartialEq + Clone + Hash, N: Node<I> + Clone> RootedTree<I, N> {
    pub fn clone_from(&self, id: I) -> Option<RootedTree<I, N>> {
        self.clone_from_with_lvl(id, None)
    }

    pub fn clone_from_with_lvl(&self, id: I, lvl: Option<u32>) -> Option<RootedTree<I, N>> {
        // Clone the root node
        if let Some(root_node) = &self.root_node {
            if root_node.id() == id && lvl.is_none() {
                return Some(self.clone());
            }
        }

        // Clone from a middle node
        let children = self.list_child_ids_with_lvl(&id, lvl);
        let mut sub_tree = RootedTree::new();

        if self.root_node.is_some() && self.root_node.as_ref().unwrap().id() == id && lvl.is_some()
        {
            sub_tree.root_node = self.root_node.clone();
        } else {
            sub_tree.root_node = self.child_nodes.get(&id).cloned();
        }

        for child_id in children {
            if let Some(node) = self.child_nodes.get(&child_id).cloned() {
                sub_tree.child_nodes.insert(child_id.clone(), node);
            }
        }

        Some(sub_tree)
    }
}

impl<I: Eq + PartialEq + Clone + Hash, N: Node<I> + Clone> Clone for RootedTree<I, N> {
    fn clone(&self) -> Self {
        let mut sub_tree = RootedTree::new();
        sub_tree.root_node = self.root_node.clone();
        sub_tree.child_nodes = self.child_nodes.clone();
        sub_tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::*;
    use crate::Config;

    #[test]
    fn clone() {
        let mut tree = RootedTree::<i32, DataNode>::new();
        tree.add_node(None, DataNode::new(1)).unwrap();

        let cloned_tree = tree.clone();

        assert!(tree == cloned_tree);
    }

    #[test]
    fn clone_root() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();

        let sub_tree = tree.clone_from(1).unwrap();

        assert!(tree == sub_tree);
    }

    #[test]
    fn clone_child() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(2), DataNode::new(3)).unwrap();
        let _tree = tree.clone();

        let sub_tree = tree.clone_from(2).unwrap();

        assert!(_tree == tree);

        assert_eq!(tree.len(), 3);
        assert_eq!(sub_tree.get_node(&2).unwrap().parent_id(), Some(1));
        assert_eq!(sub_tree.get_node(&2).unwrap().child_ids_vec(), vec![3]);
        assert_eq!(sub_tree.get_node(&3).unwrap().parent_id(), Some(2));
        assert_eq!(sub_tree.get_node(&3).unwrap().child_ids_vec(), vec![]);
    }

    #[test]
    fn clone_end_child() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(2), DataNode::new(3)).unwrap();
        let _tree = tree.clone();

        let sub_tree = tree.clone_from(3).unwrap();

        assert!(_tree == tree);

        println!("{}", sub_tree.report(&Config::default()).unwrap());

        assert_eq!(sub_tree.len(), 1);
        assert_eq!(sub_tree.get_node(&3).unwrap().parent_id(), Some(2));
        assert_eq!(sub_tree.get_node(&3).unwrap().child_ids_vec(), vec![]);
    }

    #[test]
    fn clone_with_lvl_from_root() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(2), DataNode::new(3)).unwrap();

        let sub_tree = tree.clone_from_with_lvl(1, Some(1)).unwrap();

        assert!(tree != sub_tree);
        assert_eq!(sub_tree.len(), 2);
        assert_eq!(sub_tree.get_node(&1).unwrap().parent_id(), None);
        assert_eq!(sub_tree.get_node(&1).unwrap().child_ids_vec(), vec![2]);
        assert_eq!(sub_tree.get_node(&2).unwrap().parent_id(), Some(1));
        assert_eq!(sub_tree.get_node(&2).unwrap().child_ids_vec(), vec![3]);
    }

    #[test]
    fn clone_with_lvl_from_child() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(2), DataNode::new(3)).unwrap();

        let sub_tree = tree.clone_from_with_lvl(2, Some(1)).unwrap();

        assert!(tree != sub_tree);
        assert_eq!(sub_tree.len(), 2);
        assert_eq!(sub_tree.get_node(&2).unwrap().parent_id(), Some(1));
        assert_eq!(sub_tree.get_node(&2).unwrap().child_ids_vec(), vec![3]);
        assert_eq!(sub_tree.get_node(&3).unwrap().parent_id(), Some(2));
        assert_eq!(sub_tree.get_node(&3).unwrap().child_ids_vec(), vec![]);
    }

    #[test]
    fn clone_with_lvl_from_end_child() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(2), DataNode::new(3)).unwrap();

        let sub_tree = tree.clone_from_with_lvl(3, Some(1)).unwrap();

        assert!(tree != sub_tree);
        assert_eq!(sub_tree.len(), 1);
        assert_eq!(sub_tree.get_node(&3).unwrap().parent_id(), Some(2));
        assert_eq!(sub_tree.get_node(&3).unwrap().child_ids_vec(), vec![]);
    }
}

use crate::node::Node;
use crate::{Error, Result};
use std::collections::HashMap;
use std::hash::Hash;

pub struct RootedTree<I: Eq + PartialEq + Clone, N: Node<I>> {
    pub(crate) root_node: Option<N>,
    pub(crate) child_nodes: HashMap<I, N>,
}

impl<I: Eq + PartialEq + Clone + Hash, N: Node<I>> RootedTree<I, N> {
    pub fn new() -> Self {
        Self {
            root_node: None,
            child_nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, parent_id: Option<I>, mut node: N) -> Result<()> {
        if parent_id.is_none() && self.root_node.is_some() {
            return Err(Error::RootNodeAlreadyExists);
        }
        if let Some(parent_id) = parent_id {
            if let Some(parent_node) = self.get_mut_node(&parent_id) {
                parent_node.add_child_id(node.id());
                node.set_parent_id(parent_id.clone());
                self.child_nodes.insert(node.id(), node);
            } else {
                return Err(Error::ParentNodeDoesNotExist);
            }
        } else {
            if node.parent_id().is_some() {
                return Err(Error::RootNodeHasParent);
            }
            self.root_node = Some(node);
        }
        Ok(())
    }

    pub fn get_node(&self, id: &I) -> Option<&N> {
        if let Some(node) = self.child_nodes.get(id) {
            Some(node)
        } else if let Some(root_node) = &self.root_node {
            if root_node.id() == *id {
                Some(root_node)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_mut_node(&mut self, id: &I) -> Option<&mut N> {
        if let Some(node) = self.child_nodes.get_mut(id) {
            Some(node)
        } else if let Some(root_node) = &mut self.root_node {
            if root_node.id() == *id {
                Some(root_node)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn remove_node(&mut self, id: &I) -> Option<N> {
        if let Some(node) = self.child_nodes.remove(id) {
            if let Some(parent_id) = node.parent_id() {
                if let Some(parent_node) = self.get_mut_node(&parent_id) {
                    parent_node.remove_child_id(id);
                }
            }
            Some(node)
        } else {
            self.root_node.take()
        }
    }

    pub fn len(&self) -> usize {
        if let Some(_) = &self.root_node {
            self.child_nodes.len() + 1
        } else if self.child_nodes.len() > 0 {
            unreachable!("Dag could not have child nodes without a root node")
        } else {
            0
        }
    }

    pub fn is_subtree(&self) -> bool {
        if let Some(root_node) = &self.root_node {
            root_node.parent_id().is_some()
        } else {
            false
        }
    }

    pub(crate) fn set_root_node(&mut self, node: N) {
        self.root_node = Some(node);
    }

    pub(crate) fn set_child_node(&mut self, node: N) -> Result<()> {
        if let Some(parent_id) = node.parent_id() {
            if let Some(parent_node) = self.get_node(&parent_id) {
                if !parent_node.child_ids_vec().contains(&node.id()) {
                    return Err(Error::ParentNodeDoesNotContainChild);
                } else {
                    self.child_nodes.insert(node.id(), node);
                    Ok(())
                }
            } else {
                return Err(Error::ParentNodeDoesNotExist);
            }
        } else {
            return Err(Error::ChildNodeHasNoParent);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::*;

    #[test]
    fn add_root_node() {
        let mut r_tree = RootedTree::<i32, DataNode>::new();
        let node = DataNode::new(1);
        r_tree.add_node(None, node).unwrap();
        assert_eq!(r_tree.len(), 1);

        let node = r_tree.get_node(&1).unwrap();
        assert_eq!(node.id(), 1);
        assert_eq!(node.parent_id(), None);
        assert_eq!(node.child_ids_vec(), vec![]);
    }

    #[test]
    fn add_child_to_a_root_node() {
        let mut r_tree = RootedTree::<i32, DataNode>::new();
        let node = DataNode::new(1);
        r_tree.add_node(None, node).unwrap();
        let node = DataNode::new(2);
        r_tree.add_node(Some(1), node).unwrap();
        assert_eq!(r_tree.len(), 2);

        let node_1 = r_tree.get_node(&1).unwrap();
        assert_eq!(node_1.id(), 1);
        assert_eq!(node_1.parent_id(), None);
        assert_eq!(node_1.child_ids_vec(), vec![2]);
        let node_2 = r_tree.get_node(&2).unwrap();
        assert_eq!(node_2.id(), 2);
        assert_eq!(node_2.parent_id(), Some(1));
        assert_eq!(node_2.child_ids_vec(), vec![]);
    }

    #[test]
    fn fail_to_add_2_root_nodes() {
        let mut r_tree = RootedTree::<i32, DataNode>::new();
        let node = DataNode::new(1);
        r_tree.add_node(None, node).unwrap();
        let node = DataNode::new(2);
        assert!(matches!(
            r_tree.add_node(None, node),
            Err(Error::RootNodeAlreadyExists)
        ));
    }

    #[test]
    fn fail_to_add_child_to_non_existent_parent() {
        let mut r_tree = RootedTree::<i32, DataNode>::new();
        let node = DataNode::new(1);
        assert!(matches!(
            r_tree.add_node(Some(2), node),
            Err(Error::ParentNodeDoesNotExist)
        ));
    }

    #[test]
    fn fail_to_set_root_with_an_parent_id() {
        let mut tree = RootedTree::new();
        let mut node = DataNode::new(1);
        node.set_parent_id(0);
        assert!(matches!(
            tree.add_node(None, node),
            Err(Error::RootNodeHasParent)
        ));
    }

    #[test]
    fn remove_root_node() {
        let mut r_tree = RootedTree::<i32, DataNode>::new();
        let node = DataNode::new(1);
        r_tree.add_node(None, node).unwrap();
        assert_eq!(r_tree.len(), 1);
        r_tree.remove_node(&1).unwrap();
        assert_eq!(r_tree.len(), 0);
    }

    #[test]
    fn remove_child_node() {
        let mut r_tree = RootedTree::<i32, DataNode>::new();
        let node = DataNode::new(1);
        r_tree.add_node(None, node).unwrap();
        let node = DataNode::new(2);
        r_tree.add_node(Some(1), node).unwrap();
        r_tree.remove_node(&2).unwrap();
        assert_eq!(r_tree.len(), 1);

        let node = r_tree.get_node(&1).unwrap();
        assert_eq!(node.id(), 1);
        assert_eq!(node.parent_id(), None);
        assert_eq!(node.child_ids_vec(), vec![]);
    }

    #[test]
    fn is_subtree() {
        let mut r_tree = RootedTree::<i32, DataNode>::new();
        let node = DataNode::new(1);
        r_tree.add_node(None, node).unwrap();
        assert_eq!(r_tree.is_subtree(), false);
        let mut r_tree = RootedTree::<i32, DataNode>::new();
        let mut node = DataNode::new(1);
        node.set_parent_id(2);
        r_tree.set_root_node(node);
        assert_eq!(r_tree.is_subtree(), true);
    }

    #[test]
    fn set_root_node() {
        let mut r_tree = RootedTree::<i32, DataNode>::new();
        let node = DataNode::new(1);
        r_tree.set_root_node(node);
        assert_eq!(r_tree.len(), 1);
        let node = r_tree.get_node(&1).unwrap();
        assert_eq!(node.id(), 1);
        assert_eq!(node.parent_id(), None);
        assert_eq!(node.child_ids_vec(), vec![]);
    }

    #[test]
    fn set_child_node() {
        let mut r_tree = RootedTree::<i32, DataNode>::new();
        let mut node = DataNode::new(1);
        node.add_child_id(2);
        r_tree.set_root_node(node);
        let mut node = DataNode::new(2);
        node.set_parent_id(1);
        r_tree.set_child_node(node).unwrap();
        assert_eq!(r_tree.len(), 2);
        let node = r_tree.get_node(&1).unwrap();
        assert_eq!(node.id(), 1);
        assert_eq!(node.parent_id(), None);
        assert_eq!(node.child_ids_vec(), vec![2]);
        let node = r_tree.get_node(&2).unwrap();
        assert_eq!(node.id(), 2);
        assert_eq!(node.parent_id(), Some(1));
        assert_eq!(node.child_ids_vec(), vec![]);
    }

    #[test]
    fn fail_to_set_child_node_parent_node_does_not_contain_child() {
        let mut r_tree = RootedTree::<i32, DataNode>::new();
        let mut node = DataNode::new(1);
        node.add_child_id(2);
        r_tree.set_root_node(node);
        let mut node = DataNode::new(2);
        node.set_parent_id(3);
        assert!(matches!(
            r_tree.set_child_node(node),
            Err(Error::ParentNodeDoesNotExist)
        ));
    }

    #[test]
    fn fail_to_set_child_node_parent_does_not_exist() {
        let mut r_tree = RootedTree::<i32, DataNode>::new();
        let mut node = DataNode::new(1);
        node.add_child_id(2);
        r_tree.set_root_node(node);
        let mut node = DataNode::new(2);
        node.set_parent_id(3);
        assert!(matches!(
            r_tree.set_child_node(node),
            Err(Error::ParentNodeDoesNotExist)
        ));
    }

    #[test]
    fn fail_to_set_child_node_child_node_has_no_parent() {
        let mut r_tree = RootedTree::<i32, DataNode>::new();
        let mut node = DataNode::new(1);
        node.add_child_id(2);
        r_tree.set_root_node(node);
        let node = DataNode::new(2);
        assert!(matches!(
            r_tree.set_child_node(node),
            Err(Error::ChildNodeHasNoParent)
        ));
    }
}

use crate::node::Node;
use crate::{Error, Result};
use std::collections::HashMap;
use std::hash::Hash;

pub struct RTree<I: Eq + PartialEq + Clone, N: Node<I>> {
    root_node: Option<N>,
    child_nodes: HashMap<I, N>,
}

impl<I: Eq + PartialEq + Clone + Hash, N: Node<I>> RTree<I, N> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DataNode {
        id: i32,
        parent_id: Option<i32>,
        child_ids: Vec<i32>,
    }

    impl DataNode {
        pub fn new(id: i32) -> Self {
            Self {
                id,
                parent_id: None,
                child_ids: vec![],
            }
        }
    }

    impl Node<i32> for DataNode {
        fn id(&self) -> i32 {
            self.id
        }

        fn parent_id(&self) -> Option<i32> {
            self.parent_id
        }

        fn child_ids_vec(&self) -> Vec<i32> {
            self.child_ids.clone()
        }

        fn set_parent_id(&mut self, parent: i32) {
            self.parent_id = Some(parent);
        }

        fn add_child_id(&mut self, child_id: i32) {
            self.child_ids.push(child_id);
        }

        fn remove_child_id(&mut self, child_id: &i32) {
            self.child_ids.retain(|id| id != child_id);
        }
    }

    #[test]
    fn add_root_node() {
        let mut r_tree = RTree::<i32, DataNode>::new();
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
        let mut r_tree = RTree::<i32, DataNode>::new();
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
        let mut r_tree = RTree::<i32, DataNode>::new();
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
        let mut r_tree = RTree::<i32, DataNode>::new();
        let node = DataNode::new(1);
        assert!(matches!(
            r_tree.add_node(Some(2), node),
            Err(Error::ParentNodeDoesNotExist)
        ));
    }

    #[test]
    fn remove_root_node() {
        let mut r_tree = RTree::<i32, DataNode>::new();
        let node = DataNode::new(1);
        r_tree.add_node(None, node).unwrap();
        assert_eq!(r_tree.len(), 1);
        r_tree.remove_node(&1).unwrap();
        assert_eq!(r_tree.len(), 0);
    }

    #[test]
    fn remove_child_node() {
        let mut r_tree = RTree::<i32, DataNode>::new();
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
}

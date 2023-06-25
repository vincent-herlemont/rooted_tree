use crate::node::{Id, Node};
use crate::{Error, Result};
use std::collections::HashMap;
use std::hash::Hash;

struct Dag<I: Eq + PartialEq + Hash + Clone, T: Id<I>> {
    root_node: Option<Node<I, T>>,
    child_nodes: HashMap<I, Node<I, T>>,
}

impl<I: Eq + PartialEq + Hash + Clone, T: Id<I>> Dag<I, T> {
    pub fn new() -> Self {
        Self {
            root_node: None,
            child_nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, parent_id: Option<I>, mut node: Node<I, T>) -> Result<()> {
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

    pub fn get_node(&self, id: &I) -> Option<&Node<I, T>> {
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

    pub fn get_mut_node(&mut self, id: &I) -> Option<&mut Node<I, T>> {
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

    pub fn remove_node(&mut self, id: &I) -> Option<Node<I, T>> {
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
        if let Some(root_node) = &self.root_node {
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
    use crate::node::Id;

    impl Id<i32> for i32 {
        fn id(&self) -> i32 {
            *self
        }
    }

    #[test]
    fn add_root_node() {
        let mut dag = Dag::<i32, i32>::new();
        let node = Node::new(1);
        dag.add_node(None, node).unwrap();
        assert_eq!(dag.len(), 1);

        let node = dag.get_node(&1).unwrap();
        assert_eq!(node.id(), 1);
        assert_eq!(node.parent_id(), None);
        assert_eq!(node.child_ids_vec(), vec![]);
    }

    #[test]
    fn add_child_to_a_root_node() {
        let mut dag = Dag::<i32, i32>::new();
        let node = Node::new(1);
        dag.add_node(None, node).unwrap();
        let node = Node::new(2);
        dag.add_node(Some(1), node).unwrap();
        assert_eq!(dag.len(), 2);

        let node_1 = dag.get_node(&1).unwrap();
        assert_eq!(node_1.id(), 1);
        assert_eq!(node_1.parent_id(), None);
        assert_eq!(node_1.child_ids_vec(), vec![2]);
        let node_2 = dag.get_node(&2).unwrap();
        assert_eq!(node_2.id(), 2);
        assert_eq!(node_2.parent_id(), Some(1));
        assert_eq!(node_2.child_ids_vec(), vec![]);
    }

    #[test]
    fn fail_to_add_2_root_nodes() {
        let mut dag = Dag::<i32, i32>::new();
        let node = Node::new(1);
        dag.add_node(None, node).unwrap();
        let node = Node::new(2);
        assert!(dag.add_node(None, node).is_err());
    }

    #[test]
    fn fail_to_add_child_to_non_existent_parent() {
        let mut dag = Dag::<i32, i32>::new();
        let node = Node::new(1);
        dag.add_node(None, node).unwrap();
        let node = Node::new(2);
        assert!(dag.add_node(Some(2), node).is_err());
    }

    #[test]
    fn remove_root_node() {
        let mut dag = Dag::<i32, i32>::new();
        let node = Node::new(1);
        dag.add_node(None, node).unwrap();
        assert_eq!(dag.len(), 1);
        dag.remove_node(&1);
        assert_eq!(dag.len(), 0);
    }

    #[test]
    fn remove_child_node() {
        let mut dag = Dag::<i32, i32>::new();
        let node = Node::new(1);
        dag.add_node(None, node).unwrap();
        let node = Node::new(2);
        dag.add_node(Some(1), node).unwrap();
        assert_eq!(dag.len(), 2);
        dag.remove_node(&2);
        assert_eq!(dag.len(), 1);

        let node = dag.get_node(&1).unwrap();
        assert_eq!(node.id(), 1);
        assert_eq!(node.parent_id(), None);
        assert_eq!(node.child_ids_vec(), vec![]);
    }
}

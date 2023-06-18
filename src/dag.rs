use std::collections::HashMap;
use std::hash::Hash;
use crate::node::{Id, Node};
use crate::{Result, Error};

struct Dag<I: Eq + PartialEq + Hash,T: Id<I>> {
    nodes: HashMap<I, Node<I,T>>,
}

impl <I: Eq + PartialEq + Hash,T: Id<I>> Dag<I,T> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node<I,T>) -> Result<()> {
        // let parent_id = if let Some(parent_id) = node.parent_id().is_none() {
        //     parent_id
        // } else {
        //     if  self.nodes.len() > 0 {
        //         return Err(Error::Dag("Root node already exists.".to_string()));
        //     } else {
        //
        //     }
        // };

        let parent_id:&I = match (node.parent_id(),self.nodes.len()) {
            (None, 0) => { // Insert root node
                self.nodes.insert(node.id(), node);
                return Ok(());
            },
            (None, _) => { // Root node already exists
                return Err(Error::Dag("Root node already exists.".to_string()));
            },
            (Some(parent_id), 0) => {
                return Err(Error::Dag("Root node must not have a parent.".to_string()));
            },
            (Some(parent_id), _) => {
                parent_id
            },
        };

        if !self.nodes.contains_key(parent_id) {
            return Err(Error::Dag("Parent node does not exist.".to_string()));
        }
        Ok(())
    }

    pub fn get_node(&self, id: &I) -> Option<&Node<I,T>> {
        self.nodes.get(id)
    }

    pub fn remove_node(&mut self, id: &I) -> Option<Node<I,T>> {
        self.nodes.remove(id)
    }
}


#[cfg(test)]
mod tests {
    use crate::node::Id;
    use super::*;

    impl Id<i32> for i32 {
        fn id(&self) -> i32 {
            *self
        }
    }

    #[test]
    fn add_root_node() {
        let mut dag = Dag::<i32, i32>::new();
        let node = Node::new(1);
        dag.add_node(node).unwrap();
        assert_eq!(dag.nodes.len(), 1);
    }

    #[test]
    fn fail_to_add_root_node_with_parent() {
        let mut dag = Dag::<i32, i32>::new();
        let mut node = Node::new(1);
        node.set_parent_id(2);
        assert!(dag.add_node(node).is_err());
        assert_eq!(dag.nodes.len(), 0);
    }

    #[test]
    fn fail_to_add_two_root_nodes() {
        let mut dag = Dag::<i32, i32>::new();
        let node = Node::new(1);
        dag.add_node(node).unwrap();
        let node = Node::new(2);
        assert!(dag.add_node(node).is_err());
        assert_eq!(dag.nodes.len(), 1);
    }

    #[test]
    fn fail_to_add_child_with_an_unknown_parent_node() {
        let mut dag = Dag::<i32, i32>::new();
        let node = Node::new(1);
        dag.add_node(node).unwrap();
        let mut node = Node::new(2);
        node.set_parent_id(3);
        assert!(dag.add_node(node).is_err());
        assert_eq!(dag.nodes.len(), 1);
    }
}
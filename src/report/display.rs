use crate::{Config, Node, RootedTree};
use std::fmt::Display;
use std::hash::Hash;

impl<I: Eq + PartialEq + Clone + Hash + Display + Ord + Display, N: Node<I> + Clone> Display
    for RootedTree<I, N>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lvl_string = self
            .report(&Config::default())
            .map_err(|_| std::fmt::Error)?;
        lvl_string.remove(0);
        write!(f, "{}", lvl_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::*;

    #[test]
    fn display() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(0)).unwrap();
        tree.add_node(Some(0), DataNode::new(1)).unwrap();

        let string = format!("{}", tree);
        assert_eq!(
            string,
            " 0
 └── 0 ↜ 1
"
        );
    }
}

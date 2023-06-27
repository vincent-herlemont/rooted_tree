use crate::{Node, RootedTree};
use std::fmt::Display;
use std::hash::Hash;
use unicode_width::UnicodeWidthStr;

#[derive(Clone)]
enum LvlChar {
    Space(u32),
    SolidBar(u32),
    SolidAngle(u32),
    SolidDashAngle(u32),
    SolidCross(u32),
    SolidDashCross(u32),
    DashBar(u32),
}

impl LvlChar {
    fn real_len(delta: i32, len: u32) -> usize {
        if delta.abs() as u32 >= len {
            return 0;
        }
        (len as i32 + delta) as usize
    }
}

impl Display for LvlChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LvlChar::Space(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!("    {}", " ".repeat(LvlChar::real_len(-1, *parent_len)))
                )
            }
            LvlChar::SolidBar(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!(" │  {}", " ".repeat(LvlChar::real_len(-1, *parent_len)))
                )
            }
            LvlChar::SolidAngle(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!(" └──{}", "─".repeat(LvlChar::real_len(-1, *parent_len)))
                )
            }
            LvlChar::SolidDashAngle(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!(" └╌╌╌╌╌╌{}", "╌".repeat(LvlChar::real_len(3, *parent_len)))
                )
            }
            LvlChar::SolidCross(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!(" ├──{}", "─".repeat(LvlChar::real_len(-1, *parent_len)))
                )
            }
            LvlChar::SolidDashCross(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!(" ├╌╌╌╌╌╌{}", "╌".repeat(LvlChar::real_len(3, *parent_len)))
                )
            }
            // LvlChar::DashBar(_) => write!(f, " ╎  "),
            LvlChar::DashBar(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!(" ╎  {}", " ".repeat(LvlChar::real_len(-1, *parent_len)))
                )
            }
        }
    }
}

fn get_parent_id_and_len<I: Display, N: Node<I>>(node: &N) -> (Option<I>, u32) {
    if let Some(parent_id) = node.parent_id() {
        let len = UnicodeWidthStr::width(format!("{}", parent_id).as_str());
        (Some(parent_id), len as u32)
    } else {
        (None, 0)
    }
}

impl<I: Eq + PartialEq + Clone + Hash + Display, N: Node<I>> Display for RootedTree<I, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(root) = &self.root_node {
            if let (Some(_), len) = get_parent_id_and_len(root) {
                write!(
                    f,
                    "\n{}{}",
                    LvlChar::DashBar(0),
                    self.format_node(root, vec![LvlChar::DashBar(len)], "".to_string())
                )?;
            } else {
                write!(f, "{}", self.format_node(root, vec![], "".to_string()))?;
            }
        }
        write!(f, "\n")?;
        Ok(())
    }
}

fn compute_prefixs(lvl_prefixes: &Vec<LvlChar>, suffix: String) -> String {
    let mut result = String::new();
    for (index, lvl_prefix) in lvl_prefixes.iter().enumerate() {
        if index == lvl_prefixes.len() - 1 {
            result.push_str(suffix.as_str());
        } else {
            result.push_str(lvl_prefix.to_string().as_str());
        }
    }
    result
}

impl<I: Eq + PartialEq + Clone + Hash + Display, N: Node<I>> RootedTree<I, N> {
    fn format_node(&self, node: &N, lvl_prefixes: Vec<LvlChar>, sufix: String) -> String {
        let prefix = compute_prefixs(&lvl_prefixes, sufix);
        let mut result = format!("\n{} ", prefix);

        let parent_len = if let (Some(parent_id), len) = get_parent_id_and_len(node) {
            result.push_str(&format!("{} ↜ ", parent_id));
            len
        } else {
            0
        };

        result.push_str(&format!("{}", node.id()));

        let vec_ids = node.child_ids_vec();
        let vec_ids_len = vec_ids.len();
        for (index, child_id) in vec_ids.iter().enumerate() {
            let mut lvl_prefixes = lvl_prefixes.clone();
            let current_end_branch = if index == vec_ids_len - 1 {
                lvl_prefixes.push(LvlChar::Space(parent_len));
                true
            } else {
                lvl_prefixes.push(LvlChar::SolidBar(parent_len));
                false
            };

            if let Some(child) = self.get_node(&child_id) {
                let suffix = if current_end_branch {
                    LvlChar::SolidAngle(parent_len).to_string()
                } else {
                    LvlChar::SolidCross(parent_len).to_string()
                };
                result.push_str(&self.format_node(child, lvl_prefixes.clone(), suffix));
            } else {
                let suffix = if current_end_branch {
                    LvlChar::SolidDashAngle(parent_len).to_string()
                } else {
                    LvlChar::SolidDashCross(parent_len).to_string()
                };
                let prefix = compute_prefixs(&lvl_prefixes, suffix);
                result.push_str(&format!("\n{} {}", prefix, child_id));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::*;

    #[test]
    fn b_test_large_root_sub_tree_key() {
        let mut tree = RootedTree::new();
        let mut node = DataNode::new(1);
        node.set_parent_id(2222222);
        tree.set_root_node(node);

        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(1), DataNode::new(3)).unwrap();

        // println!("{}", tree);
        println!("{}", tree);
    }

    #[test]
    fn b_test_large_root_key_partial() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1111111111)).unwrap();
        let mut node = DataNode::new(22222);
        node.add_child_id(3);
        node.add_child_id(4);
        node.add_child_id(5);
        tree.add_node(Some(1111111111), node).unwrap();

        tree.add_node(Some(22222), DataNode::new(4)).unwrap();
        println!("{}", tree);
    }

    #[test]
    fn b_test_large_root_key() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1111111111)).unwrap();
        tree.add_node(Some(1111111111), DataNode::new(22222))
            .unwrap();
        tree.add_node(Some(22222), DataNode::new(3)).unwrap();
        tree.add_node(Some(22222), DataNode::new(4)).unwrap();
        tree.add_node(Some(22222), DataNode::new(5)).unwrap();
        println!("{}", tree);
    }

    #[test]
    fn test_debug_one_child() {
        let mut tree = RootedTree::new();
        let mut node = DataNode::new(1);
        node.add_child_id(2);
        tree.set_root_node(node);

        let mut node = DataNode::new(2);
        node.set_parent_id(1);
        tree.set_child_node(node).unwrap();
        println!("{}", tree);
    }

    #[test]
    fn test_debug_subtree() {
        let mut tree = RootedTree::new();
        let mut node = DataNode::new(1);
        node.set_parent_id(0);
        tree.set_root_node(node);
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        println!("{}", tree);
    }

    #[test]
    fn partial_childs() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        let mut node = DataNode::new(2);
        node.set_parent_id(1);
        node.add_child_id(3);
        node.add_child_id(4);
        node.add_child_id(5);
        tree.add_node(Some(1), node).unwrap();

        tree.add_node(Some(2), DataNode::new(4)).unwrap();

        println!("{}", tree);
    }

    #[test]
    fn test_two_childs() {
        let mut tree = RootedTree::new();
        let mut node = DataNode::new(1);
        node.add_child_id(2);
        node.add_child_id(3);
        tree.set_root_node(node);

        let mut node = DataNode::new(2);
        node.set_parent_id(1);
        tree.set_child_node(node).unwrap();

        let mut node = DataNode::new(3);
        node.set_parent_id(1);
        tree.set_child_node(node).unwrap();

        println!("{}", tree);
        //        assert_eq!(
        //            format!("{:?}", tree),
        //            format!(
        //                "
        // 1
        // ├── 1 ↜ 2
        // └── 1 ↜ 3
        // "
        //            )
        //        );
    }

    #[test]
    fn test_nested_childs() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(2), DataNode::new(3)).unwrap();
        tree.add_node(Some(3), DataNode::new(4)).unwrap();
        tree.add_node(Some(4), DataNode::new(5)).unwrap();

        println!("{}", tree);
    }

    #[test]
    fn test_nested_childs_2() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(1), DataNode::new(3)).unwrap();
        tree.add_node(Some(1), DataNode::new(4)).unwrap();

        tree.add_node(Some(2), DataNode::new(5)).unwrap();
        tree.add_node(Some(2), DataNode::new(6)).unwrap();
        tree.add_node(Some(2), DataNode::new(7)).unwrap();

        tree.add_node(Some(6), DataNode::new(8)).unwrap();
        tree.add_node(Some(6), DataNode::new(9)).unwrap();
        tree.add_node(Some(6), DataNode::new(10)).unwrap();

        tree.add_node(Some(4), DataNode::new(11)).unwrap();
        tree.add_node(Some(4), DataNode::new(12)).unwrap();
        tree.add_node(Some(4), DataNode::new(13)).unwrap();

        tree.add_node(Some(10), DataNode::new(14)).unwrap();
        tree.add_node(Some(10), DataNode::new(15)).unwrap();
        tree.add_node(Some(10), DataNode::new(16)).unwrap();

        println!("{}", tree);
    }

    #[test]
    fn test_subrooted_tree_nested_childs_2() {
        let mut tree = RootedTree::new();
        let mut node = DataNode::new(1);
        node.set_parent_id(0);
        tree.set_root_node(node);
        tree.add_node(Some(1), DataNode::new(22)).unwrap();
        tree.add_node(Some(1), DataNode::new(3)).unwrap();
        tree.add_node(Some(1), DataNode::new(4)).unwrap();

        tree.add_node(Some(22), DataNode::new(5)).unwrap();
        tree.add_node(Some(22), DataNode::new(6)).unwrap();
        tree.add_node(Some(22), DataNode::new(7)).unwrap();

        tree.add_node(Some(6), DataNode::new(8)).unwrap();
        tree.add_node(Some(6), DataNode::new(9)).unwrap();
        tree.add_node(Some(6), DataNode::new(10)).unwrap();

        tree.add_node(Some(4), DataNode::new(11)).unwrap();
        tree.add_node(Some(4), DataNode::new(12)).unwrap();
        let mut node = DataNode::new(13);
        node.add_child_id(17);
        node.add_child_id(18);
        tree.add_node(Some(4), node).unwrap();

        tree.add_node(Some(10), DataNode::new(14)).unwrap();
        tree.add_node(Some(10), DataNode::new(15)).unwrap();
        tree.add_node(Some(10), DataNode::new(16)).unwrap();

        println!("{}", tree);
    }
}

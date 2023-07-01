mod display;
mod lvl_string;

pub use display::*;

use crate::{Node, RootedTree};
use lvl_string::*;
use std::fmt::Display;
use std::fmt::Write;
use std::hash::Hash;
use thiserror::Error;
use unicode_width::UnicodeWidthStr;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Formatting error")]
    Formatting(#[from] std::fmt::Error),
}

#[derive(Clone, PartialEq, Eq)]
pub enum ChildWrap {
    Top,
    Bottom,
}

impl Default for ChildWrap {
    fn default() -> Self {
        Self::Bottom
    }
}

#[derive(Default, Clone)]
pub struct Config<I> {
    max_children: Option<u32>,
    child_wrap: ChildWrap,
    // (node_id, max_lvl_around_node)
    select_node: Option<(I, u32)>,
}

#[derive(Clone)]
pub struct Meta<I> {
    select_nodes: Vec<I>,
}

impl<I> Default for Meta<I> {
    fn default() -> Self {
        Self {
            select_nodes: vec![],
        }
    }
}

impl<I: Eq + PartialEq + Clone + Hash + Display + Ord, N: Node<I> + Clone> RootedTree<I, N> {
    pub fn report(&self, config: &Config<I>) -> Result<String> {
        if let Some((node_id, lvl)) = &config.select_node {
            let sub_lvl = lvl + 1 / 2;
            let parent_ids = self.list_parent_ids_with_lvl(&node_id, Some(sub_lvl.clone()));
            let root_id = parent_ids.last().unwrap_or(node_id);
            if let Some(temp_rooted_tree) = self.clone_from_with_lvl(root_id.clone(), Some(sub_lvl))
            {
                let mut select_nodes = vec![node_id.clone()];
                select_nodes.extend(parent_ids);
                let meta = Meta { select_nodes };
                return Self::_report(&temp_rooted_tree, config, &meta);
            }
        }
        Self::_report(self, config, &Meta::default())
    }

    fn _report(
        rooted_tree: &RootedTree<I, N>,
        config: &Config<I>,
        meta: &Meta<I>,
    ) -> Result<String> {
        let mut out = String::new();
        if let Some(root) = &rooted_tree.root_node {
            if let (Some(_), len) = get_parent_id_and_len(root) {
                write!(
                    out,
                    "\n{}{}",
                    LvlChar::DashBar(0),
                    rooted_tree.format_node(
                        &config,
                        root,
                        vec![LvlChar::DashBar(len)],
                        "".to_string(),
                        meta
                    )
                )?;
            } else {
                write!(
                    out,
                    "{}",
                    rooted_tree.format_node(&config, root, vec![], "".to_string(), meta)
                )?;
            }
        }
        write!(out, "\n")?;
        Ok(out)
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

fn compute_prefixes(lvl_prefixes: &Vec<LvlChar>, suffix: String) -> String {
    let mut result = String::new();
    if lvl_prefixes.is_empty() {
        result.push_str(suffix.as_str());
        return result;
    }
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
    fn format_node(
        &self,
        config: &Config<I>,
        node: &N,
        lvl_prefixes: Vec<LvlChar>,
        suffix: String,
        meta: &Meta<I>,
    ) -> String {
        let prefix = compute_prefixes(&lvl_prefixes, suffix);
        let mut result = format!("\n{} ", prefix);

        let parent_len = if let (Some(parent_id), len) = get_parent_id_and_len(node) {
            result.push_str(&format!("{} ↜ ", parent_id));
            len
        } else {
            0
        };

        result.push_str(&format!("{}", node.id()));

        let mut vec_ids = node.child_ids_vec();
        let mut vec_ids_len = vec_ids.len();

        // Wrap top
        if let Some(max_child) = config.max_children {
            if vec_ids_len > max_child as usize {
                let add_wrap_top = if !meta.select_nodes.is_empty() {
                    let mut index_select_nodes = 0;
                    loop {
                        if let Some(index_node_id) = vec_ids
                            .iter()
                            .position(|x| x == meta.select_nodes.get(index_select_nodes).unwrap())
                        {
                            let index_node_stop_wrap = index_node_id - max_child as usize / 2;
                            if index_node_stop_wrap == 0 {
                                break false;
                            } else {
                                vec_ids = vec_ids[index_node_stop_wrap..vec_ids_len].to_vec();
                                vec_ids_len = vec_ids.len();
                                break true;
                            }
                        } else {
                            index_select_nodes += 1;
                            if index_select_nodes == meta.select_nodes.len() {
                                break false;
                            }
                        }
                    }
                } else {
                    if let ChildWrap::Top = config.child_wrap {
                        vec_ids = vec_ids[max_child as usize..vec_ids_len].to_vec();
                        vec_ids_len = vec_ids.len();
                        true
                    } else {
                        false
                    }
                };

                if add_wrap_top {
                    let mut lvl_prefixes = lvl_prefixes.clone();
                    lvl_prefixes.push(LvlChar::DashBar(parent_len));
                    lvl_prefixes.push(LvlChar::Empty);
                    let prefix = compute_prefixes(&lvl_prefixes, "".to_string());
                    result.push_str(&format!("\n{}", prefix));
                }
            }
        }

        for (index, child_id) in vec_ids.iter().enumerate() {
            let mut lvl_prefixes = lvl_prefixes.clone();

            // Wrap bottom
            if let Some(max_child) = config.max_children {
                if ChildWrap::Bottom == config.child_wrap || config.select_node.is_some() {
                    if index == max_child as usize {
                        lvl_prefixes.push(LvlChar::DashBar(parent_len));
                        lvl_prefixes.push(LvlChar::Empty);
                        let prefix = compute_prefixes(&lvl_prefixes, "".to_string());
                        result.push_str(&format!("\n{}", prefix));
                        break;
                    }
                }
            }

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
                result.push_str(&self.format_node(
                    &config,
                    child,
                    lvl_prefixes.clone(),
                    suffix,
                    meta,
                ));
            } else {
                let suffix = if current_end_branch {
                    LvlChar::SolidDashAngle(parent_len).to_string()
                } else {
                    LvlChar::SolidDashCross(parent_len).to_string()
                };
                let prefix = compute_prefixes(&lvl_prefixes, suffix);
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
    fn select_node_center_challenge_simple() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(1), DataNode::new(3)).unwrap();
        tree.add_node(Some(1), DataNode::new(4)).unwrap();

        tree.add_node(Some(4), DataNode::new(5)).unwrap();
        tree.add_node(Some(4), DataNode::new(6)).unwrap();
        tree.add_node(Some(4), DataNode::new(7)).unwrap();
        tree.add_node(Some(4), DataNode::new(8)).unwrap();

        tree.add_node(Some(1), DataNode::new(5)).unwrap();
        tree.add_node(Some(1), DataNode::new(6)).unwrap();

        println!("{}", tree.report(&Config::default()).unwrap());

        let mut config = Config::default();
        config.select_node = Some((4, 1));
        config.max_children = Some(2);

        println!("{}", tree.report(&config).unwrap());
    }

    #[test]
    fn select_node_center_challenge_hidden() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(11)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(2), DataNode::new(3)).unwrap();
        tree.add_node(Some(2), DataNode::new(7)).unwrap();
        tree.add_node(Some(2), DataNode::new(8)).unwrap();
        tree.add_node(Some(3), DataNode::new(4)).unwrap();
        tree.add_node(Some(4), DataNode::new(5)).unwrap();
        tree.add_node(Some(5), DataNode::new(6)).unwrap();

        println!("{}", tree.report(&Config::default()).unwrap());

        let mut config = Config::default();
        config.select_node = Some((8, 10));
        config.max_children = Some(1);
        println!("{}", tree.report(&config).unwrap());
    }

    #[test]
    fn select_node_center_challenge_parent_nodes() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(2), DataNode::new(3)).unwrap();
        tree.add_node(Some(3), DataNode::new(4)).unwrap();
        tree.add_node(Some(4), DataNode::new(5)).unwrap();
        tree.add_node(Some(5), DataNode::new(6)).unwrap();

        println!("{}", tree.report(&Config::default()).unwrap());

        let mut config = Config::default();
        config.select_node = Some((4, 0));
        config.max_children = Some(1);
        println!("{}", tree.report(&config).unwrap());

        let mut config = Config::default();
        config.select_node = Some((4, 1));
        config.max_children = Some(1);
        println!("{}", tree.report(&config).unwrap());

        let mut config = Config::default();
        config.select_node = Some((4, 2));
        config.max_children = Some(1);
        println!("{}", tree.report(&config).unwrap());

        let mut config = Config::default();
        config.select_node = Some((4, 3));
        config.max_children = Some(1);
        println!("{}", tree.report(&config).unwrap());

        let mut config = Config::default();
        config.select_node = Some((4, 4));
        config.max_children = Some(1);
        println!("{}", tree.report(&config).unwrap());

        let mut config = Config::default();
        config.select_node = Some((4, 5));
        config.max_children = Some(1);
        println!("{}", tree.report(&config).unwrap());
    }

    #[test]
    fn select_node_lvl_challenge() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(1), DataNode::new(3)).unwrap();
        tree.add_node(Some(3), DataNode::new(4)).unwrap();
        tree.add_node(Some(3), DataNode::new(5)).unwrap();
        tree.add_node(Some(5), DataNode::new(6)).unwrap();
        tree.add_node(Some(5), DataNode::new(7)).unwrap();
        tree.add_node(Some(7), DataNode::new(8)).unwrap();
        tree.add_node(Some(7), DataNode::new(9)).unwrap();
        tree.add_node(Some(9), DataNode::new(10)).unwrap();
        tree.add_node(Some(9), DataNode::new(11)).unwrap();

        println!("{}", tree.report(&Config::default()).unwrap());

        let mut config = Config::default();
        config.select_node = Some((5, 1));
        println!("{}", tree.report(&config).unwrap());

        let mut config = Config::default();
        config.select_node = Some((5, 2));
        println!("{}", tree.report(&config).unwrap());
    }

    #[test]
    fn max_child_1_lvl() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(1), DataNode::new(3)).unwrap();
        tree.add_node(Some(1), DataNode::new(4)).unwrap();
        tree.add_node(Some(1), DataNode::new(5)).unwrap();

        let mut config = Config::default();
        config.max_children = Some(2);

        println!("{}", tree.report(&config.clone()).unwrap());

        config.child_wrap = ChildWrap::Top;

        println!("{}", tree.report(&config).unwrap());
    }

    #[test]
    fn max_child_2_lvl() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(2), DataNode::new(3)).unwrap();
        tree.add_node(Some(2), DataNode::new(4)).unwrap();

        tree.add_node(Some(4), DataNode::new(6)).unwrap();
        tree.add_node(Some(4), DataNode::new(7)).unwrap();
        tree.add_node(Some(4), DataNode::new(8)).unwrap();

        tree.add_node(Some(2), DataNode::new(5)).unwrap();
        tree.add_node(Some(1), DataNode::new(10)).unwrap();
        tree.add_node(Some(1), DataNode::new(11)).unwrap();

        // tree.add_node(Some(1), DataNode::new(3)).unwrap();
        // tree.add_node(Some(1), DataNode::new(4)).unwrap();
        // tree.add_node(Some(1), DataNode::new(5)).unwrap();

        let mut config = Config::default();
        config.max_children = Some(2);

        println!("{}", tree.report(&config.clone()).unwrap());

        // config.child_wrap = ChildWrap::Top;
        //
        // println!("{}", tree.report(&config).unwrap());
    }

    #[test]
    fn b_test_large_root_sub_tree_key() {
        let mut tree = RootedTree::new();
        let mut node = DataNode::new(1);
        node.set_parent_id(2222222);
        tree.set_root_node(node);

        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(1), DataNode::new(3)).unwrap();

        println!("{}", tree.report(&Config::default()).unwrap());
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
        println!("{}", tree.report(&Config::default()).unwrap());
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
        println!("{}", tree.report(&Config::default()).unwrap());
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
        println!("{}", tree.report(&Config::default()).unwrap());
    }

    #[test]
    fn test_debug_subtree() {
        let mut tree = RootedTree::new();
        let mut node = DataNode::new(1);
        node.set_parent_id(0);
        tree.set_root_node(node);
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        println!("{}", tree.report(&Config::default()).unwrap());
    }

    #[test]
    fn partial_children() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        let mut node = DataNode::new(2);
        node.set_parent_id(1);
        node.add_child_id(3);
        node.add_child_id(4);
        node.add_child_id(5);
        tree.add_node(Some(1), node).unwrap();

        tree.add_node(Some(2), DataNode::new(4)).unwrap();

        println!("{}", tree.report(&Config::default()).unwrap());
    }

    #[test]
    fn test_two_children() {
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

        println!("{}", tree.report(&Config::default()).unwrap());
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
    fn test_nested_children() {
        let mut tree = RootedTree::new();
        tree.add_node(None, DataNode::new(1)).unwrap();
        tree.add_node(Some(1), DataNode::new(2)).unwrap();
        tree.add_node(Some(2), DataNode::new(3)).unwrap();
        tree.add_node(Some(3), DataNode::new(4)).unwrap();
        tree.add_node(Some(4), DataNode::new(5)).unwrap();

        println!("{}", tree.report(&Config::default()).unwrap());
    }

    #[test]
    fn test_nested_children_2() {
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

        println!("{}", tree.report(&Config::default()).unwrap());
    }

    #[test]
    fn test_subrooted_tree_nested_children_2() {
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

        let config = Config::default();

        println!("{}", tree.report(&config).unwrap());
    }
}

use camino::Utf8Path;
use camino::Utf8PathBuf;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait Id<T> {
    fn id(&self) -> T;
}

#[derive(Debug)]
pub struct NodeImplementation<I, T: Id<I>> {
    inner: T,
    parent_id: Option<I>,
    child_ids: HashSet<I>,
}

impl<I: Hash + Eq + PartialEq + Clone, T: Id<I>> NodeImplementation<I, T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: value,
            parent_id: None,
            child_ids: HashSet::new(),
        }
    }

    pub fn parent_id(&self) -> Option<I> {
        self.parent_id.clone()
    }

    pub fn child_ids(&self) -> HashSet<I> {
        self.child_ids.clone()
    }

    pub fn child_ids_vec(&self) -> Vec<I> {
        self.child_ids.iter().cloned().collect::<Vec<I>>()
    }

    pub fn set_parent_id(&mut self, parent: I) {
        self.parent_id = Some(parent);
    }

    pub fn add_child_id(&mut self, child_id: I) {
        self.child_ids.insert(child_id);
    }

    pub fn remove_child_id(&mut self, child_id: &I) {
        self.child_ids.remove(child_id);
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn id(&self) -> I {
        self.inner.id()
    }
}

impl Id<String> for Utf8PathBuf {
    fn id(&self) -> String {
        self.to_string()
    }
}

impl Id<String> for Utf8Path {
    fn id(&self) -> String {
        self.to_string()
    }
}

fn from_paths(paths: Vec<&str>) -> HashMap<String, NodeImplementation<String, Utf8PathBuf>> {
    let root_path = Utf8PathBuf::from("/");
    let mut map: HashMap<String, NodeImplementation<String, Utf8PathBuf>> = HashMap::new();
    map.insert(root_path.id(), NodeImplementation::new(root_path.clone()));

    let paths = paths
        .iter()
        .map(|path| Utf8PathBuf::from(path))
        .collect::<Vec<Utf8PathBuf>>();

    for path in paths {
        let mut current_path = root_path.clone();
        for component in path.components() {
            current_path = current_path.join(component);
            if map.contains_key(&current_path.id()) {
                continue;
            } else {
                let mut node = NodeImplementation::new(current_path.clone());
                if let Some(parent_path) = current_path.parent() {
                    if let Some(parent_node) = map.get_mut(&parent_path.id()) {
                        node.set_parent_id(parent_path.id());
                        parent_node.add_child_id(current_path.id());
                    }
                }
                map.insert(current_path.id(), node);
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_paths() {
        let paths = vec![
            "/home/username/Downloads",
            "/home/username/Documents",
            "/home/username/Documents/Books",
            "/home/username/Documents/Books/Programming",
        ];

        let tree = from_paths(paths);
        assert_eq!(tree.len(), 7);
        assert_eq!(tree.get(&"/".to_string()).unwrap().parent_id, None);
        assert_eq!(tree.get(&"/".to_string()).unwrap().child_ids.len(), 1);
        assert!(tree
            .get(&"/".to_string())
            .unwrap()
            .child_ids
            .get(&"/home".to_string())
            .is_some());

        assert_eq!(
            tree.get(&"/home".to_string()).unwrap().parent_id,
            Some("/".to_string())
        );
        assert_eq!(tree.get(&"/home".to_string()).unwrap().child_ids.len(), 1);
        assert!(tree
            .get(&"/home".to_string())
            .unwrap()
            .child_ids
            .get(&"/home/username".to_string())
            .is_some());

        assert_eq!(
            tree.get(&"/home/username".to_string()).unwrap().parent_id,
            Some("/home".to_string())
        );
        assert_eq!(
            tree.get(&"/home/username".to_string())
                .unwrap()
                .child_ids
                .len(),
            2
        );
        assert!(tree
            .get(&"/home/username".to_string())
            .unwrap()
            .child_ids
            .get(&"/home/username/Downloads".to_string())
            .is_some());
        assert!(tree
            .get(&"/home/username".to_string())
            .unwrap()
            .child_ids
            .get(&"/home/username/Documents".to_string())
            .is_some());

        assert_eq!(
            tree.get(&"/home/username/Downloads".to_string())
                .unwrap()
                .parent_id,
            Some("/home/username".to_string())
        );
        assert_eq!(
            tree.get(&"/home/username/Downloads".to_string())
                .unwrap()
                .child_ids
                .len(),
            0
        );

        assert_eq!(
            tree.get(&"/home/username/Documents".to_string())
                .unwrap()
                .parent_id,
            Some("/home/username".to_string())
        );
        assert_eq!(
            tree.get(&"/home/username/Documents".to_string())
                .unwrap()
                .child_ids
                .len(),
            1
        );
        assert!(tree
            .get(&"/home/username/Documents".to_string())
            .unwrap()
            .child_ids
            .get(&"/home/username/Documents/Books".to_string())
            .is_some());

        assert_eq!(
            tree.get(&"/home/username/Documents/Books".to_string())
                .unwrap()
                .parent_id,
            Some("/home/username/Documents".to_string())
        );
        assert_eq!(
            tree.get(&"/home/username/Documents/Books".to_string())
                .unwrap()
                .child_ids
                .len(),
            1
        );
        assert!(tree
            .get(&"/home/username/Documents/Books".to_string())
            .unwrap()
            .child_ids
            .get(&"/home/username/Documents/Books/Programming".to_string())
            .is_some());

        assert_eq!(
            tree.get(&"/home/username/Documents/Books/Programming".to_string())
                .unwrap()
                .parent_id,
            Some("/home/username/Documents/Books".to_string())
        );
        assert_eq!(
            tree.get(&"/home/username/Documents/Books/Programming".to_string())
                .unwrap()
                .child_ids
                .len(),
            0
        );
    }

    #[test]
    fn test_from_paths_empty() {
        // Testing with no paths
        let paths: Vec<&str> = vec![];
        let tree = from_paths(paths);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.get(&"/".to_string()).unwrap().parent_id, None);
        assert_eq!(tree.get(&"/".to_string()).unwrap().child_ids.len(), 0);
    }

    #[test]
    fn test_from_paths_root() {
        // Testing with only root
        let paths = vec!["/"];
        let tree = from_paths(paths);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.get(&"/".to_string()).unwrap().parent_id, None);
        assert_eq!(tree.get(&"/".to_string()).unwrap().child_ids.len(), 0);
    }

    #[test]
    fn test_from_paths_single_level() {
        // Testing with single level paths
        let paths = vec!["/home", "/bin", "/etc"];
        let tree = from_paths(paths);
        assert_eq!(tree.len(), 4);
        assert_eq!(tree.get(&"/".to_string()).unwrap().parent_id, None);
        assert_eq!(tree.get(&"/".to_string()).unwrap().child_ids.len(), 3);
        assert!(tree
            .get(&"/".to_string())
            .unwrap()
            .child_ids
            .get(&"/home".to_string())
            .is_some());
        assert!(tree
            .get(&"/".to_string())
            .unwrap()
            .child_ids
            .get(&"/bin".to_string())
            .is_some());
        assert!(tree
            .get(&"/".to_string())
            .unwrap()
            .child_ids
            .get(&"/etc".to_string())
            .is_some());
    }

    #[test]
    fn test_from_paths_same_branch() {
        // Testing with multiple paths in the same branch
        let paths = vec!["/home/username/Documents", "/home/username/Downloads"];
        let tree = from_paths(paths);
        assert_eq!(tree.len(), 5);
        assert_eq!(
            tree.get(&"/home/username".to_string()).unwrap().parent_id,
            Some("/home".to_string())
        );
        assert_eq!(
            tree.get(&"/home/username".to_string())
                .unwrap()
                .child_ids
                .len(),
            2
        );
        assert!(tree
            .get(&"/home/username".to_string())
            .unwrap()
            .child_ids
            .get(&"/home/username/Documents".to_string())
            .is_some());
        assert!(tree
            .get(&"/home/username".to_string())
            .unwrap()
            .child_ids
            .get(&"/home/username/Downloads".to_string())
            .is_some());
    }

    #[test]
    fn test_from_paths_mixed() {
        // Testing with a mix of single and multi level paths
        let paths = vec!["/home/username/Documents", "/bin"];
        let tree = from_paths(paths);
        assert_eq!(tree.len(), 5);
        assert_eq!(tree.get(&"/".to_string()).unwrap().parent_id, None);
        assert_eq!(tree.get(&"/".to_string()).unwrap().child_ids.len(), 2);
        assert!(tree
            .get(&"/".to_string())
            .unwrap()
            .child_ids
            .get(&"/home".to_string())
            .is_some());
        assert!(tree
            .get(&"/".to_string())
            .unwrap()
            .child_ids
            .get(&"/bin".to_string())
            .is_some());
    }
}

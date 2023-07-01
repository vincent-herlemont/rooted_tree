use crate::Node;

#[derive(Eq, PartialEq, Clone)]
pub struct DataNode {
    pub(crate) id: i32,
    pub(crate) parent_id: Option<i32>,
    pub(crate) child_ids: Vec<i32>,
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
        if self.child_ids.contains(&child_id) {
            return;
        }
        self.child_ids.push(child_id);
    }

    fn remove_child_id(&mut self, child_id: &i32) {
        self.child_ids.retain(|id| id != child_id);
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct DataNodeString {
    pub(crate) id: String,
    pub(crate) parent_id: Option<String>,
    pub(crate) child_ids: Vec<String>,
}

impl DataNodeString {
    pub fn new(id: &str) -> Self {
        Self {
            id: String::from(id),
            parent_id: None,
            child_ids: vec![],
        }
    }
}

impl Node<String> for DataNodeString {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn parent_id(&self) -> Option<String> {
        self.parent_id.clone()
    }

    fn child_ids_vec(&self) -> Vec<String> {
        self.child_ids.clone()
    }

    fn set_parent_id(&mut self, parent: String) {
        self.parent_id = Some(parent);
    }

    fn add_child_id(&mut self, child_id: String) {
        if self.child_ids.contains(&child_id) {
            return;
        }
        self.child_ids.push(child_id);
    }

    fn remove_child_id(&mut self, child_id: &String) {
        self.child_ids.retain(|id| id != child_id);
    }
}

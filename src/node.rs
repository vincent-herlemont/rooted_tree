pub trait Node<I> {
    fn id(&self) -> I;
    fn parent_id(&self) -> Option<I>;
    fn child_ids_vec(&self) -> Vec<I>;
    fn set_parent_id(&mut self, parent: I);
    fn add_child_id(&mut self, child_id: I);
    fn remove_child_id(&mut self, child_id: &I);
}

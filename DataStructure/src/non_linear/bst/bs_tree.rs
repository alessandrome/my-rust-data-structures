use super::node::BSNode;

pub struct BSTree<T> {
    root: Option<BSNode<T>>,
}

impl<T> BSTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
    
    pub fn insert(&mut self, node: BSNode<T>) {}
}

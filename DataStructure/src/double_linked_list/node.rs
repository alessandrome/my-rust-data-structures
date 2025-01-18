use std::ptr::NonNull;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub successor: Option<NonNull<Node<T>>>,
    pub predecessor: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            successor: None,
            predecessor: None,
        }
    }
}

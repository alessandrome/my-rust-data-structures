mod node;
mod tests;

use std::ptr::NonNull;
use node::Node;

pub struct DoubleLinkedList<T> {
    length: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> DoubleLinkedList<T> {
        DoubleLinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }
}

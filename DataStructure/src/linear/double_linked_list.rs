mod node;
mod tests;

use node::Node;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct DoubleLinkedList<T> {
    length: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> DoubleLinkedList<T> {
        DoubleLinkedList {
            length: 0,
            head: None, // First node of the list
            tail: None, // Last node of the list
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn append(&mut self, other: T) {
        let new_node = Box::into_raw(Box::new(Node::new(other)));
        if self.length == 0 {
            self.head = NonNull::new(new_node);
            self.tail = NonNull::new(new_node);
            self.length += 1;
            return;
        }
    }
}

pub mod node;

use node::Node;
use std::any::type_name;
use std::fmt::Display;
use std::ptr::NonNull;

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    length: usize,
}

impl<T: Display> SinglyLinkedList<T> {
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList {
            tail: None,
            head: None,
            length: 0,
        }
    }
    pub fn push_back(&mut self, item: T) {
        let mut new_node = Box::new(Node::new(item));
        let new_nod_ptr = NonNull::new(&mut *new_node);
        if self.tail.is_none() {
            self.head = Some(new_node);
        } else {
            unsafe {
                self.tail.unwrap().as_mut().next = Some(new_node);
            }
        }
        self.tail = new_nod_ptr;
        self.length += 1;
    }
    pub fn pop_back(&mut self) -> Option<T> {
        if self.length > 0 {
            self.length -= 1;

            if let Some(node) = &self.head {
                if node.next.is_none() {
                    self.tail = None;
                    return Some(self.head.take()?.value);
                }
            }

            let mut second_last_node: _ = self.head.as_mut().unwrap();
            let mut last_node = &second_last_node.next;
            second_last_node = second_last_node.next.as_mut().unwrap();  // Questo causerà un errore
            last_node = &second_last_node.next;

            while let Some(last_node_box) = last_node {
                if last_node_box.next.is_some() {
                    second_last_node = second_last_node.next.as_mut().unwrap();
                } else {
                    break;
                }
                last_node = &second_last_node.next;
            }
            self.tail = Some(NonNull::from(&mut **second_last_node));
            let removed = second_last_node.next.take().unwrap();
            return Some(removed.value);
        }
        None
    }
    pub fn push_front(&mut self, item: T) {
        let new_node = Node::new(item);
        // if self.head.is_none() {
        //     self.head = Some(Box::new(new_node));
        // }
    }
}

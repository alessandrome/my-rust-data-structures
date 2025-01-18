pub mod node;
mod tests;

use node::Node;
use std::any::type_name;
use std::fmt::{write, Display, Formatter};
use std::ptr::NonNull;
use std::ops::{Index, IndexMut};

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    length: usize,
}

impl<T: PartialEq> SinglyLinkedList<T> {
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList {
            tail: None,
            head: None,
            length: 0,
        }
    }

    /// Push a new element to attach to the list tail
    ///
    /// # Arguments
    ///
    /// * `item`: Element T you want to append
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ll = singly_linked_list::<i32>::new();
    /// ll.push_back(5); // Add value '5' as last element of the list
    /// ```
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
        let mut new_node = Box::new(Node::new(item));
        let new_nod_ptr = NonNull::new(&mut *new_node);
        if self.tail.is_none() {
            self.tail = new_nod_ptr;
        } else {
            new_node.next = self.head.take();
        }
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.length > 0 {
            self.length -= 1;

            if let Some(node) = &self.head {
                if node.next.is_none() {
                    self.tail = None;
                    return Some(self.head.take()?.value);
                }
            }
            let old_head = self.head.take()?;
            self.head = old_head.next;
            return Some(old_head.value);
        }
        None
    }

    pub fn get(&self, index: usize) -> Option<T> where T: Clone {
        if index < self.length {
            let mut node_box = self.head.as_ref().unwrap();
            for _ in 0..index {
                node_box = node_box.next.as_ref().unwrap();
            }
            return Some(node_box.value.clone());
        }
        None
    }

    pub fn get_ref(&self, index: usize) -> Option<&T> {
        if index < self.length {
            let mut node_box = self.head.as_ref().unwrap();
            for _ in 0..index {
                node_box = node_box.next.as_ref().unwrap();
            }
            return Some(&node_box.value);
        }
        None
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.length {
            let mut node_box = self.head.as_mut().unwrap();
            for _ in 0..index {
                node_box = node_box.next.as_mut().unwrap();
            }
            return Some(&mut node_box.value);
        }
        None
    }

    pub fn find(&self, value: &T) -> Option<usize> {
        let mut node_opt = self.head.as_ref();
        for i in 0..self.length {
            let node_box = node_opt.unwrap();
            if node_box.value == *value {
                return Some(i);
            }
            node_opt = node_box.next.as_ref();
        }
        None
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.length {
            // If index is 0, you're popping the head
            if index == 0 {
                return self.pop_front();
            }

            // Else if index is the tail pop back
            if index == self.length - 1 {
                return self.pop_back();
            }

            // Else iterate to reach predecessor of node to remove
            let mut pre_node_opt: _ = self.head.as_mut();
            for i in 0..index - 1 {
                pre_node_opt = pre_node_opt.unwrap().next.as_mut();
            }
            // Unwrap and change ownerships to bypass node to remove
            let pre_node_box = pre_node_opt.unwrap();
            let to_remove = pre_node_box.next.take().unwrap();
            pre_node_box.next = to_remove.next; // Moving successor of node to remove as next of the predecessor to remove

            // Update length of the list
            self.length -= 1;
            return Some(to_remove.value);
        }
        None
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn print(&self) {
        if self.length > 0 {
            let mut node_opt = &self.head;
            while let Some(node) = node_opt {
                print!("({})", node.value);
                if node.next.is_some() {
                    print!(" -> ");
                }
                node_opt = &node.next;
            }
            println!("");
        } else {
            println!("()");
        }
    }
}

impl<T> Index<usize> for SinglyLinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.length {
            panic!("Index {} out of bounds, list length is {}", index, self.length);
        }
        let mut node_box = self.head.as_ref().unwrap();
        for _ in 0..index {
            node_box = node_box.next.as_ref().unwrap();
        }
        &node_box.value
    }
}

impl<T> IndexMut<usize> for SinglyLinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.length {
            panic!("Index {} out of bounds, list length is {}", index, self.length);
        }
        let mut node_box = self.head.as_mut().unwrap();
        for _ in 0..index {
            node_box = node_box.next.as_mut().unwrap();
        }
        &mut node_box.value
    }
}

impl<T: Display> Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut node_box = self.head.as_ref().unwrap();
        for i in 0..self.length {
            write!(f, "{}", node_box.value)?;
            if i < self.length - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

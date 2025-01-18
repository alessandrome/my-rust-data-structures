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

    pub fn from_array(array: &[T]) -> DoubleLinkedList<T>
    where
        T: Clone,
    {
        let mut list: DoubleLinkedList<T> = DoubleLinkedList::new();
        for val in array {
            list.append(val.clone());
        }
        list
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn append(&mut self, value: T) {
        // Creating a new node on heap memory
        let mut new_node = Box::new(Node::new(value));

        if self.length == 0 {
            // List is empty - new node is head and tail at the same time
            let new_node_ptr = Box::into_raw(new_node);
            self.head = NonNull::new(new_node_ptr);
            self.tail = NonNull::new(new_node_ptr);
        } else {
            // List is not empty - Append to the old tail - The new node is the new tail
            let mut old_tail;
            unsafe {
                old_tail = &mut *self.tail.as_mut().unwrap().as_mut();
            }
            new_node.predecessor = NonNull::new(old_tail);
            let new_node_ptr = Box::into_raw(new_node);
            old_tail.successor = NonNull::new(new_node_ptr);
            self.tail = NonNull::new(new_node_ptr);
        }

        // Increase size of list by one
        self.length += 1;
    }

    pub fn prepend(&mut self, value: T) {
        // Creating a new node on heap memory
        let mut new_node = Box::new(Node::new(value));

        if self.length == 0 {
            // List is empty - new node is head and tail at the same time
            let new_node_ptr = Box::into_raw(new_node);
            self.head = NonNull::new(new_node_ptr);
            self.tail = NonNull::new(new_node_ptr);
        } else {
            // List is not empty - Append to the old tail - The new node is the new tail
            let mut old_head;
            unsafe {
                old_head = &mut *self.head.as_mut().unwrap().as_mut();
            }
            new_node.successor = NonNull::new(old_head);
            let new_node_ptr = Box::into_raw(new_node);
            old_head.predecessor = NonNull::new(new_node_ptr);
            self.head = NonNull::new(new_node_ptr);
        }

        // Increase size of list by one
        self.length += 1;
    }

    pub fn insert(&mut self, value: T, index: usize) -> Result<(), String> {
        // Creating a new node on heap memory
        if index > self.length {
            return Err(format!("index {} out of bounds", index));
        }
        if index == 0 {
            self.prepend(value);
        } else if index == self.length {
            self.append(value);
        } else {
            // List is not empty - Append to the old tail - The new node is the new tail
            // Creating a new node on heap memory
            let mut new_node = Box::new(Node::new(value));

            // Iterate till node in the previous position where push the item
            let mut previous_node = unsafe { self.head.as_mut().unwrap().as_mut() };
            for _ in 0..index - 1 {
                previous_node = unsafe { previous_node.successor.as_mut().unwrap().as_mut() };
            }


            new_node.successor = previous_node.successor;  // New node successor is now successor of predecessor (index - 1)
            new_node.predecessor = NonNull::new(previous_node);  // New node successor is node in pos. index - 1
            let new_node_ptr = Box::into_raw(new_node);
            previous_node.successor = NonNull::new(new_node_ptr);  // The previous node successor is the new node
            unsafe { (*new_node_ptr).successor.as_mut().unwrap().as_mut().predecessor = NonNull::new(new_node_ptr) };  // Predecessor of successor is the new node
        }

        // Increase size of list by one
        self.length += 1;
        Ok(())
    }

    pub fn tail_ref(&self) -> Option<&T> {
        if let Some(tail) = &self.tail {
            return Some(&unsafe { tail.as_ref() }.value);
        }
        None
    }

    pub fn tail_mut(&mut self) -> Option<&mut T> {
        if let Some(tail) = &mut self.tail {
            return Some(&mut unsafe { tail.as_mut() }.value);
        }
        None
    }

    pub fn head_ref(&self) -> Option<&T> {
        if let Some(head) = &self.head {
            return Some(&unsafe { head.as_ref() }.value);
        }
        None
    }

    pub fn head_mut(&mut self) -> Option<&mut T> {
        if let Some(head) = &mut self.head {
            return Some(&mut unsafe { head.as_mut() }.value);
        }
        None
    }
}

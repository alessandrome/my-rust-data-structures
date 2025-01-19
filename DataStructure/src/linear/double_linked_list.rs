mod node;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod benchs;

use node::Node;
use std::ptr::NonNull;
use std::ops::{Drop};

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

    pub fn pop_head(&mut self) -> Result<T, &'static str> {
        if self.length == 0 {
            return Err("Pop head on empty list");
        }
        let box_head = unsafe { Box::from_raw(self.head.unwrap().as_ptr()) };
        self.head = box_head.successor;
        if let Some(head) = &mut self.head {
            unsafe { head.as_mut().predecessor = None };
        } else {
            // Popped value was the unique one in the list, now it is empty
            self.tail = None;
        }
        self.length -= 1;
        Ok(box_head.value)
    }

    pub fn pop_tail(&mut self) -> Result<T, &'static str> {
        if self.length == 0 {
            return Err("Pop tail on empty list");
        }
        let box_tail = unsafe { Box::from_raw(self.tail.unwrap().as_ptr()) };
        self.tail = box_tail.predecessor;
        if let Some(tail) = &mut self.tail {
            unsafe { tail.as_mut().successor = None };
        } else {
            // Popped value was the unique one in the list, now it is empty
            self.head = None;
        }
        self.length -= 1;
        Ok(box_tail.value)
    }

    pub fn remove(&mut self, index: usize) -> Result<T, String> {
        if self.length == 0 {
            return Err(format!("Removing index {}: out of bound", index));
        }
        if index >= self.length {
            return Err(format!("Index {} out of bounds (Length is {})", index, self.length));
        }
        if index == 0 {
            return Ok(self.pop_head()?);
        }
        if index == self.length - 1 {
            return Ok(self.pop_tail()?);
        }
        let mut previous_node = unsafe { self.head.as_mut().unwrap().as_mut() };
        for _ in 0..index - 1 {
            previous_node = unsafe { previous_node.successor.as_mut().unwrap().as_mut() };
        }

        let mut to_remove = unsafe {Box::from_raw(previous_node.successor.unwrap().as_ptr()) };
        previous_node.successor = to_remove.successor;
        unsafe { previous_node.successor.as_mut().unwrap().as_mut().predecessor = to_remove.predecessor };

        self.length -= 1;
        Ok(to_remove.value)
    }

    pub fn get_head_ref(&self) -> Result<&T, &'static str> {
        if self.length == 0 {
            return Err("Get head on empty list");
        }
        Ok(unsafe { &self.head.as_ref().unwrap().as_ref().value })
    }

    pub fn get_head_mut(&mut self) -> Result<&mut T, &'static str> {
        if self.length == 0 {
            return Err("Get head on empty list");
        }
         Ok(unsafe { &mut self.head.as_mut().unwrap().as_mut().value })
    }

    pub fn get_tail_ref(&self) -> Result<&T, &'static str> {
        if self.length == 0 {
            return Err("Get tail on empty list");
        }
        Ok(unsafe { &self.tail.as_ref().unwrap().as_ref().value })
    }

    pub fn get_tail_mut(&mut self) -> Result<&mut T, &'static str> {
        if self.length == 0 {
            return Err("Get tail on empty list");
        }
        Ok(unsafe { &mut self.tail.as_mut().unwrap().as_mut().value })
    }

    pub fn get_ref(&mut self, index: usize) -> Result<&T, String> {
        if index >= self.length {
            return Err(format!("Index {} out of bounds (Length  {})", index, self.length));
        }

        if index == 0 {
            return Ok(self.get_head_ref()?);
        }
        if index == self.length - 1 {
            return Ok(self.get_tail_ref()?);
        }

        let mut node = unsafe { self.head.as_ref().unwrap().as_ref() };
        for _ in 0..index {
            node = unsafe { node.successor.as_ref().unwrap().as_ref() };
        }
        Ok(&node.value)
    }

    pub fn get_mut(&mut self, index: usize) -> Result<&mut T, String> {
        if index >= self.length {
            return Err(format!("Index {} out of bounds (Length  {})", index, self.length));
        }

        if index == 0 {
            return Ok(self.get_head_mut()?);
        }
        if index == self.length - 1 {
            return Ok(self.get_tail_mut()?);
        }

        let mut node = unsafe { self.head.as_mut().unwrap().as_mut() };
        for _ in 0..index {
            node = unsafe { node.successor.as_mut().unwrap().as_mut() };
        }
        Ok(&mut node.value)
    }
}

impl<T> Drop for DoubleLinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head;

        while let Some(node) = current {
            unsafe {
                // Take next node for next iteration
                current = node.as_ref().successor;

                // Rebuild box with Node pointer and drop it
                drop(Box::from_raw(node.as_ptr()));
            }
        }
    }
}

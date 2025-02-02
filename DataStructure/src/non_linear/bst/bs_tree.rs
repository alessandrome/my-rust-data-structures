use std::cmp::{PartialOrd, PartialEq};
use super::node::BSNode;

pub struct BSTree<T: PartialOrd + PartialEq> {
    root: Option<Box<BSNode<T>>>,
    size: usize,
}

impl<T: PartialOrd + PartialEq> BSTree<T> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, value: T) {
        let new_boxed_node = Box::new(BSNode::new(value));
        if self.root.is_none() {
            // Tree is empty, the value will be the tree's root
            self.root = Some(new_boxed_node);
        } else {
            // Tree must be traversed and then inserted the node
            let mut checking_boxed_node = self.root.as_mut().unwrap();
            while true {
                if new_boxed_node.value == checking_boxed_node.value {
                    //Value already exists in the tree - No insert
                    return;
                }
                if new_boxed_node.value < checking_boxed_node.value {
                    // Must insert or traverse left subtree
                    let left_node_opt = checking_boxed_node.left_mut();
                    match left_node_opt {
                        None => {
                           *left_node_opt = Some(new_boxed_node);
                            self.size += 1;
                            break;
                        }
                        Some(left_node) => {
                            checking_boxed_node = left_node;
                        }
                    }
                } else {
                    // Must insert or traverse right subtree
                    let right_node_opt = checking_boxed_node.right_mut();
                    match right_node_opt {
                        None => {
                            *right_node_opt = Some(new_boxed_node);
                            self.size += 1;
                            break;
                        }
                        Some(right_node) => {
                            checking_boxed_node = right_node;
                        }
                    }
                }
            }
        }
    }

    pub fn find(&self, value: T) -> Option<&T> {
        let mut checking_boxed_node = self.root.as_ref();
        while let Some(node) = checking_boxed_node {
            if node.value == value {
                return Some(&node.value);
            }
            if value < node.value {
                checking_boxed_node = node.left().as_ref();
            } else {
                checking_boxed_node = node.right().as_ref();
            }
        }
        None
    }
}

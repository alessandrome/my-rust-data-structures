use std::cmp::{PartialOrd, PartialEq};
use std::fmt::{write, Display, Formatter};
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
            self.size += 1;
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

    fn find_min_node<'a>(root: &'a Option<Box<BSNode<T>>>) -> Option<&'a Option<Box<BSNode<T>>>> {
        match root {
            None => None,
            Some(root_node) => {
                match root_node.left() {
                    None => Some(root),
                    Some(_) => Self::find_min_node(root_node.left()),
                }
            }
        }
    }
    fn find_min_node_mut<'a>(root: &'a mut Option<Box<BSNode<T>>>) -> Option<&'a mut Option<Box<BSNode<T>>>> {
        match root {
            None => None,
            Some(_) => {
                let is_none = root.as_mut().unwrap().left().is_none();
                if is_none {
                    Some(root)
                } else {
                    Self::find_min_node_mut(root.as_mut().unwrap().left_mut())
                }
            }
        }
    }

    fn find_max_node<'a>(root: &'a Option<Box<BSNode<T>>>) -> Option<&'a Option<Box<BSNode<T>>>> {
        match root {
            None => None,
            Some(root_node) => {
                match root_node.right() {
                    None => Some(root),
                    Some(_) => Self::find_min_node(root_node.right()),
                }
            }
        }
    }
    fn find_max_node_mut<'a>(root: &'a mut Option<Box<BSNode<T>>>) -> Option<&'a mut Option<Box<BSNode<T>>>> {
        match root {
            None => None,
            Some(_) => {
                let is_none = root.as_mut().unwrap().right().is_none();
                if is_none {
                    Some(root)
                } else {
                    Self::find_min_node_mut(root.as_mut().unwrap().right_mut())
                }
            }
        }
    }

    fn find_node(&self, value: &T) -> &Option<Box<BSNode<T>>> {
        let mut checking_boxed_node = &self.root;
        // USe of _ to not assign a mut ref that will put the new checking_boxed_node ref invalid as another one has been used for a variable that could edit the content of referred item
        while let Some(_) = checking_boxed_node {
            let node = checking_boxed_node.as_ref().unwrap();
            if node.value == *value {
                break;
            }
            if *value < node.value {
                checking_boxed_node = node.left();
            } else {
                checking_boxed_node = node.right();
            }
        }
        checking_boxed_node
    }

    fn find_node_mut(&mut self, value: &T) -> &mut Option<Box<BSNode<T>>> {
        let mut checking_boxed_node = &mut self.root;
        // Use of _ to not assign a mut ref that will put the new checking_boxed_node ref invalid as another one has been used for a variable that could edit the content of referred item
        while let Some(_) = checking_boxed_node {
            let node_value = &checking_boxed_node.as_ref().unwrap().value;
            if *node_value == *value {
                break;
            }
            if *value < *node_value {
                checking_boxed_node = checking_boxed_node.as_mut().unwrap().left_mut();
            } else {
                checking_boxed_node = checking_boxed_node.as_mut().unwrap().right_mut();
            }
        }
        checking_boxed_node
    }

    pub fn find(&self, value: &T) -> Option<&T> {
        match self.find_node(&value) {
            None => None,
            Some(node) => Some(&node.value)
        }
    }

    pub fn remove(&mut self, value: &T) -> Option<T> {
        let mut node_opt = self.find_node_mut(value);
        let mut return_val = None;
        if node_opt.is_none() {
            return return_val;
        }

        // Take out node to remove - Will be dropped at the end of the function
        let mut removed_node = node_opt.take().unwrap();
        let left_is_some = removed_node.left().is_some();
        let right_is_some = removed_node.right().is_some();

        let replace_node = if left_is_some {
            let max_node_opt = Self::find_max_node_mut(removed_node.left_mut()).unwrap();
            Some(max_node_opt.take().unwrap())
        } else if right_is_some {
            let min_node_opt = Self::find_min_node_mut(removed_node.right_mut()).unwrap();
            Some(min_node_opt.take().unwrap())
        } else {
            None
        };

        *node_opt = replace_node;
        return_val = Some(removed_node.value);

        self.size -= 1;
        return_val
    }

    pub fn in_order_values_builder<'a>(root: Option<&'a Box<BSNode<T>>>, vec: &mut Vec<&'a T>) {
        if let Some(node) = root {
            Self::in_order_values_builder(node.left().as_ref(), vec);
            vec.push(&node.value);
            Self::in_order_values_builder(node.right().as_ref(), vec);
        }
    }
    pub fn in_order_values(&self) -> Vec<&T> {
        let mut vec = Vec::new();
        Self::in_order_values_builder(self.root.as_ref(), &mut vec);
        vec
    }

    pub fn pre_order_values_builder<'a>(root: Option<&'a Box<BSNode<T>>>, vec: &mut Vec<&'a T>) {
        if let Some(node) = root {
            vec.push(&node.value);
            Self::in_order_values_builder(node.left().as_ref(), vec);
            Self::in_order_values_builder(node.right().as_ref(), vec);
        }
    }
    pub fn pre_order_values(&self) -> Vec<&T> {
        let mut vec = Vec::new();
        Self::pre_order_values_builder(self.root.as_ref(), &mut vec);
        vec
    }

    pub fn post_order_values_builder<'a>(root: Option<&'a Box<BSNode<T>>>, vec: &mut Vec<&'a T>) {
        if let Some(node) = root {
            Self::in_order_values_builder(node.left().as_ref(), vec);
            Self::in_order_values_builder(node.right().as_ref(), vec);
            vec.push(&node.value);
        }
    }
    pub fn post_order_values(&self) -> Vec<&T> {
        let mut vec = Vec::new();
        Self::post_order_values_builder(self.root.as_ref(), &mut vec);
        vec
    }
}

impl<T: Display + PartialOrd + PartialEq> BSTree<T> {
    pub fn in_order_str(&self) -> String {
        let mut str = "[".to_string();
        let values = self.in_order_values();
        for i in 0..values.len() {
            str.push_str(&values[i].to_string());
            if i != values.len() - 1 {
                str.push_str(", ");
            }
        }
        str.push(']');
        str
    }
}

impl<T: Display + PartialOrd> Display for BSTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.in_order_str())
    }
}

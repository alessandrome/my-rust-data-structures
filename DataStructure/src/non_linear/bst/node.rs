pub struct BSNode<T> {
    pub value: T,
    left: Option<Box<BSNode<T>>>,
    right: Option<Box<BSNode<T>>>,
}

impl<T> BSNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
    
    pub fn left(&self) -> &Option<Box<BSNode<T>>> {
        &self.left
    }
    pub fn left_mut(&mut self) -> &mut Option<Box<BSNode<T>>> {
        &mut self.left
    }
    
    pub fn right(&self) -> &Option<Box<BSNode<T>>> {
        &self.right
    }
    pub fn right_mut(&mut self) -> &mut Option<Box<BSNode<T>>> {
        &mut self.right
    }
}

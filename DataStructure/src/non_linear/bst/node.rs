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
}

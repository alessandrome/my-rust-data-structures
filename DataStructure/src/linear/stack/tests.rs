use super::{Stack, STACK_START_SIZE};

#[test]
fn test_new() {
    let stack = Stack::<i32>::new();
    assert!(stack.is_empty());
    assert_eq!(stack.length(), 0);
    assert_eq!(stack.buffer_size(), STACK_START_SIZE);
}

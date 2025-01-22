use super::{Stack, STACK_SIZE_INCREMENT, STACK_START_SIZE};

const STACK_ARRAY: [i32; 9] = [0, -1, 2, -3, 4, -5, 6, -7, 8];

fn create_stack() -> Stack<i32> {
    let mut stack = Stack::<i32>::new();
    for i in (0..STACK_ARRAY.len()).rev() {
        stack.push(STACK_ARRAY[i]);
    }
    stack
}

#[test]
fn test_new() {
    let stack = Stack::<i32>::new();
    assert!(stack.is_empty());
    assert_eq!(stack.length(), 0);
    assert_eq!(stack.buffer_size(), STACK_START_SIZE);
}

#[test]
fn test_pop() {
    let mut stack = create_stack();
    for i in 0..STACK_ARRAY.len() {
        let popped = stack.pop();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap(), STACK_ARRAY[i]);
    }
    assert!(stack.is_empty());
    assert!(stack.pop().is_none());
}

#[test]
fn test_push() {
    let mut stack = Stack::<i32>::new();
    for i in 0..STACK_ARRAY.len() {
        stack.push(STACK_ARRAY[i]);
        assert_eq!(stack.length(), i + 1);
    }
}

#[test]
fn test_top() {
    let mut stack = Stack::<i32>::new();
    for i in 0..STACK_ARRAY.len() {
        stack.push(STACK_ARRAY[i]);
        let top = stack.top();
        assert!(top.is_some());
        assert_eq!(*top.unwrap(), STACK_ARRAY[i]);
    }
}

#[test]
fn test_increment_buffer() {
    let mut stack = create_stack();
    assert_eq!(stack.buffer_size(), STACK_START_SIZE);
    assert!(!stack.buffer_is_full());
    stack.push(90);
    assert!(stack.buffer_is_full());
    stack.push(91);
    assert_eq!(stack.buffer_size(), STACK_START_SIZE + STACK_SIZE_INCREMENT);
    assert!(!stack.buffer_is_full());
}

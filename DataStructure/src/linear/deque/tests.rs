use super::{Deque, DEQUE_START_SIZE, DEQUE_SIZE_INCREMENT};

const DEQUE_ARRAY: [i32; 9] = [0, -1, 2, -3, 4, -5, 6, -7, 8];
const DEQUE_DISPLAY: &'static str = "Head -> [0, -1, 2, -3, 4, -5, 6, -7, 8] <- Tail";

fn create_deque() -> Deque<i32> {
    let mut deque = Deque::<i32>::new();
    for i in 0..DEQUE_ARRAY.len() {
        deque.push_back(DEQUE_ARRAY[i]);
    }
    deque
}

#[test]
fn test_new() {
    let deque = Deque::<i32>::new();
    assert!(deque.is_empty());
    assert_eq!(deque.length(), 0);
    assert_eq!(deque.buffer_size(), DEQUE_START_SIZE);
}

#[test]
fn test_pop_front() {
    let mut deque = create_deque();
    let mut count = 0_usize;
    while !deque.is_empty() {
        let popped = deque.pop_front();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap(), DEQUE_ARRAY[count]);
        count += 1;
    }
    assert!(deque.is_empty());
    assert_eq!(count, 9);
    assert!(deque.pop_front().is_none());
}

#[test]
fn test_pop_back() {
    let mut deque = create_deque();
    let mut count = 0_usize;
    while !deque.is_empty() {
        let popped = deque.pop_back();
        assert!(popped.is_some());
        count += 1;
        assert_eq!(popped.unwrap(), DEQUE_ARRAY[DEQUE_ARRAY.len() - count]);
    }
    assert!(deque.is_empty());
    assert_eq!(count, 9);
    assert!(deque.pop_back().is_none());
}

#[test]
fn test_push_front() {
    let mut deque = Deque::<i32>::new();
    assert!(deque.head().is_none());
    assert!(deque.head_mut().is_none());
    for i in 0..DEQUE_ARRAY.len() {
        deque.push_front(DEQUE_ARRAY[i]);
        assert_eq!(deque.length(), i + 1);
        assert_eq!(*deque.head().unwrap(), DEQUE_ARRAY[i]);
        assert_eq!(*deque.tail().unwrap(), DEQUE_ARRAY[0]);
    }
}

#[test]
fn test_push_back() {
    let mut deque = Deque::<i32>::new();
    assert!(deque.tail().is_none());
    assert!(deque.tail_mut().is_none());
    for i in 0..DEQUE_ARRAY.len() {
        deque.push_back(DEQUE_ARRAY[i]);
        assert_eq!(deque.length(), i + 1);
        assert_eq!(*deque.tail().unwrap(), DEQUE_ARRAY[i]);
        assert_eq!(*deque.head().unwrap(), DEQUE_ARRAY[0]);
    }
}

#[test]
fn test_head() {
    let mut deque = create_deque();
    let mut head = deque.head_mut().unwrap();
    assert_eq!(*head, 0);
    *head = 5000;
    assert_eq!(*deque.head().unwrap(), 5000);
    assert_eq!(*deque.head_mut().unwrap(), 5000);
}

#[test]
fn test_tail() {
    let mut deque = create_deque();
    let mut tail = deque.tail_mut().unwrap();
    assert_eq!(*tail, 8);
    *tail = 5000;
    assert_eq!(*deque.tail().unwrap(), 5000);
    assert_eq!(*deque.tail_mut().unwrap(), 5000);
}

#[test]
fn test_increment_buffer() {
    let mut deque = create_deque();
    assert_eq!(deque.buffer_size(), DEQUE_START_SIZE);
    assert!(!deque.buffer_is_full());
    deque.push_back(90);
    assert!(deque.buffer_is_full());
    deque.push_back(91);
    assert_eq!(deque.buffer_size(), DEQUE_START_SIZE + DEQUE_SIZE_INCREMENT);
    assert!(!deque.buffer_is_full());
    assert_eq!(*deque.tail().unwrap(), 91);
}

#[test]
fn test_display() {
    let mut deque = create_deque();
    let s = format!("{}", deque);
    assert_eq!(s, DEQUE_DISPLAY.to_string());
}

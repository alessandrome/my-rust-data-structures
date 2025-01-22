use super::{Queue, QUEUE_SIZE_INCREMENT, QUEUE_START_SIZE};

const QUEUE_ARRAY: [i32; 9] = [0, -1, 2, -3, 4, -5, 6, -7, 8];
const QUEUE_DISPLAY: &'static str = "Head -> [0, -1, 2, -3, 4, -5, 6, -7, 8] <- Tail";

fn create_queue() -> Queue<i32> {
    let mut queue = Queue::<i32>::new();
    for i in 0..QUEUE_ARRAY.len() {
        queue.push(QUEUE_ARRAY[i]);
    }
    queue
}

#[test]
fn test_new() {
    let queue = Queue::<i32>::new();
    assert!(queue.is_empty());
    assert_eq!(queue.length(), 0);
    assert_eq!(queue.buffer_size(), QUEUE_START_SIZE);
}

#[test]
fn test_pop() {
    let mut queue = create_queue();
    let mut count = 0_usize;
    while !queue.is_empty() {
        let popped = queue.pop();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap(), QUEUE_ARRAY[count]);
        count += 1;
    }
    assert!(queue.is_empty());
    assert_eq!(count, 9);
    assert!(queue.pop().is_none());
}

#[test]
fn test_push() {
    let mut queue = Queue::<i32>::new();
    for i in 0..QUEUE_ARRAY.len() {
        queue.push(QUEUE_ARRAY[i]);
        assert_eq!(queue.length(), i + 1);
    }
}

#[test]
fn test_head() {
    let mut queue = Queue::<i32>::new();
    assert!(queue.head().is_none());
    assert!(queue.head_mut().is_none());
    for i in 0..QUEUE_ARRAY.len() {
        queue.push(QUEUE_ARRAY[i]);
        let head = queue.head();
        assert!(head.is_some());
        assert_eq!(*head.unwrap(), QUEUE_ARRAY[0]);
    }
    let mut head = queue.head_mut().unwrap();
    *head = 5000;
    assert_eq!(*queue.head().unwrap(), 5000);
}

#[test]
fn test_increment_buffer() {
    let mut queue = create_queue();
    assert_eq!(queue.buffer_size(), QUEUE_START_SIZE);
    assert!(!queue.buffer_is_full());
    queue.push(90);
    assert!(queue.buffer_is_full());
    queue.push(91);
    assert_eq!(queue.buffer_size(), QUEUE_START_SIZE + QUEUE_SIZE_INCREMENT);
    assert!(!queue.buffer_is_full());
}

#[test]
fn test_display() {
    let mut queue = create_queue();
    let s = format!("{}", queue);
    assert_eq!(s, QUEUE_DISPLAY.to_string());
}

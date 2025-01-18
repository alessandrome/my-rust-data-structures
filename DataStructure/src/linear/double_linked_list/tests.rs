use super::DoubleLinkedList;

fn create_list() -> DoubleLinkedList<i32>{
    DoubleLinkedList::from_array(&[-5, 0, 3, 15, 16, 18, -20, -15, -3, -5, 5])
}
fn create_empty_list() -> DoubleLinkedList<i32>{
    DoubleLinkedList::new()
}

#[test]
fn test_append() {
    let mut list: DoubleLinkedList<i32> = create_empty_list();
    list.append(3);
    assert_eq!(*list.tail_ref().unwrap(), 3);
    list.append(4);
    assert_eq!(*list.tail_ref().unwrap(), 4);
    assert_eq!(*list.head_ref().unwrap(), 3);
}

#[test]
fn test_prepend() {
    let mut list: DoubleLinkedList<i32> = create_empty_list();
    list.prepend(3);
    assert_eq!(*list.head_ref().unwrap(), 3);
    list.prepend(4);
    assert_eq!(*list.head_ref().unwrap(), 4);
    assert_eq!(*list.tail_ref().unwrap(), 3);
}

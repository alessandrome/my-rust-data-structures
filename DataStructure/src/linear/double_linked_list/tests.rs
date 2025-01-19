use super::DoubleLinkedList;

const FROM_ARRAY: [i32; 11] = [-5, 0, 3, 15, 16, 18, -20, -15, -3, -8, 5];

fn create_list() -> DoubleLinkedList<i32> {
    DoubleLinkedList::from_array(&FROM_ARRAY)
}
fn create_empty_list() -> DoubleLinkedList<i32> {
    DoubleLinkedList::new()
}

#[test]
fn test_new() {
    let list = create_empty_list();
    assert_eq!(list.length(), 0);
    assert_eq!(list.is_empty(), true);
}

#[test]
fn test_from_array() {
    let list = create_list();
    assert_eq!(list.length(), 11);
    assert_eq!(list.is_empty(), false);
    assert_eq!(*list.head_ref().unwrap(), -5);
    assert_eq!(*list.tail_ref().unwrap(), 5);
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

#[test]
fn test_pop() {
    let mut list = create_list();
    assert_eq!(list.pop_tail().unwrap(), 5);
    assert_eq!(list.pop_tail().unwrap(), -8);
    assert_eq!(list.pop_head().unwrap(), -5);
    assert_eq!(list.pop_head().unwrap(), 0);
    assert_eq!(list.length(), FROM_ARRAY.len() - 4usize);
}

#[test]
fn test_get() {
    let mut list = create_list();
    assert_eq!(*list.get_tail_ref().unwrap(), 5);
    assert_eq!(*list.get_tail_ref().unwrap(), 5);
    assert_eq!(*list.get_head_ref().unwrap(), -5);
    assert_eq!(*list.get_head_ref().unwrap(), -5);
    assert_eq!(*list.get_ref(5).unwrap(), FROM_ARRAY[5]);
    assert_eq!(*list.get_ref(5).unwrap(), FROM_ARRAY[5]);
    let mut val = list.get_mut(5).unwrap();
    assert_eq!(*val, FROM_ARRAY[5]);
    *val += 5;
    assert_eq!(*list.get_mut(5).unwrap(), FROM_ARRAY[5] + 5);
    assert_eq!(*list.get_ref(5).unwrap(), FROM_ARRAY[5] + 5);
    assert_eq!(list.length(), FROM_ARRAY.len());
}

#[test]
fn test_get_edge_cases() {
    let mut list = create_list();
    let last_index = FROM_ARRAY.len() - 1;
    assert_eq!(*list.get_ref(0).unwrap(), FROM_ARRAY[0]);
    assert_eq!(*list.get_ref(last_index).unwrap(), FROM_ARRAY[last_index]);
    let mut h = list.get_mut(0).unwrap();
    assert_eq!(*h, FROM_ARRAY[0]);
    *h += 5;
    assert_eq!(*list.get_mut(0).unwrap(), FROM_ARRAY[0] + 5);
    assert_eq!(*list.get_ref(0).unwrap(), FROM_ARRAY[0] + 5);

    let mut t = list.get_mut(last_index).unwrap();
    assert_eq!(*t, FROM_ARRAY[last_index]);
    *t += 44;
    assert_eq!(*list.get_mut(last_index).unwrap(), FROM_ARRAY[last_index] + 44);
    assert_eq!(*list.get_ref(last_index).unwrap(), FROM_ARRAY[last_index] + 44);
    assert_eq!(list.length(), FROM_ARRAY.len());
}

#[test]
fn test_get_out_of_bounds() {
    let mut list = create_list();
    assert!(list.get_ref(FROM_ARRAY.len()).is_err());
}

#[test]
fn test_pop_out_of_bounds() {
    let mut list = create_empty_list();
    assert!(list.pop_head().is_err());
    assert!(list.pop_tail().is_err());
}

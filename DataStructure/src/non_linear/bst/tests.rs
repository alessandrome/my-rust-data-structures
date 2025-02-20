use crate::non_linear::bst::bs_tree::BSTree;

const TREE_VALUES: [i32; 9] = [5, -1, -2, -3, 7, -9, 0, 10, 4];
const TREE_DISPLAY: &'static str = "[-9, -3, -2, -1, 0, 4, 5, 7, 10]";
const TREE_DISPLAY_REMOVED: &'static str = "[-9, -3, -2, -1, 4, 5, 7, 10]";

fn create_empty_tree() -> BSTree<i32> {
    BSTree::<i32>::new()
}

fn create_tree() -> BSTree<i32> {
    let mut tree = BSTree::<i32>::new();
    for val in TREE_VALUES {
        tree.insert(val);
    }
    assert_eq!(tree.size(), TREE_VALUES.len());
    tree
}

#[test]
fn test_find() {
    let tree = create_tree();
    let result = tree.find(&7);
    assert!(result.is_some());
    assert_eq!(*result.unwrap(), 7);
    let result = tree.find(&-545);
    assert!(result.is_none());
}

#[test]
fn test_insert() {
    let mut tree = create_tree();
    let start_size = tree.size();
    let result = tree.insert(99);
    assert_eq!(tree.size(), start_size + 1);
    let result = tree.find(&99);
    assert_eq!(tree.size(), start_size + 1);
}

#[test]
fn test_remove() {
    let mut tree = create_tree();
    let start_size = tree.size();
    let result = tree.remove(&0);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 0);
    assert_eq!(tree.size(), start_size - 1);
    let result = tree.remove(&0);
    assert!(result.is_none());
    assert_eq!(tree.size(), start_size - 1);
}

#[test]
fn test_display() {
    let mut tree = create_tree();
    assert_eq!(format!("{}", tree), TREE_DISPLAY);
    let _ = tree.remove(&0);
    assert_eq!(format!("{}", tree), TREE_DISPLAY_REMOVED);
}
mod singly_linked_list;

use std::any::type_name;
use singly_linked_list::{SinglyLinkedList};


fn print_type_of<T>(_: &T) {
    println!("Il tipo è: {}", type_name::<T>());
}
struct Ex {
    pub v: i32,
}

impl Drop for Ex {
    fn drop(&mut self) {
        println!("Drop ex");
    }
}

fn test(mut op: Option<Box<Ex>>) {
    if let Some(taken) = op.take() {
        println!("OK");
    }
    eprintln!("End \"if let\"");
}

fn main() {
    let mut ob: Option<Box<Ex>> = Some(Box::new(Ex{v:22}));
    println!("Val: {}", ob.as_ref().unwrap().v);
    if let Some(taken) = &ob {
        println!("OK {}", taken.v);
    }
    println!("Is None: {}", ob.is_none());

    let mut ll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(3);
    let r = ll.pop_back();
    println!("Test pop: {}", r.is_some());
}

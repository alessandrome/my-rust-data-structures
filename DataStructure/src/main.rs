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
    {
        let mut ob: Option<Box<Ex>> = Some(Box::new(Ex { v: 22 }));
        println!("Val: {}", ob.as_ref().unwrap().v);
        if let Some(taken) = &ob {
            println!("OK {}", taken.v);
        }
        println!("Is None: {}", ob.is_none());
    }
    println!("\nTrying pushes and pops");
    let mut ll: SinglyLinkedList<i32> = SinglyLinkedList::<i32>::new();
    ll.print();
    ll.push_back(1);
    ll.print();
    ll.push_back(2);
    ll.print();
    ll.push_back(3);
    ll.print();
    let r = ll.pop_back();
    ll.print();
    ll.push_front(5);
    ll.print();
    ll.push_front(6);
    ll.print();
    ll.push_back(7);
    ll.print();
    ll.pop_back();
    ll.print();
    ll.pop_front();
    ll.print();
    ll.push_back(7);
    ll.push_front(8);
    ll.print();
    println!();
    println!("Iterate with .len and using .get_ref");
    for i in 0..ll.len() {
        print!("{}", ll.get_ref(i).unwrap());
        if i != ll.len() - 1 {
            print!(" -> ");
        }
    }
    println!();
    // println!("Test pop: {}", r.is_some());
}

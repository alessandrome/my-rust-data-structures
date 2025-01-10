mod singly_linked_list;

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
}

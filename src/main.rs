use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

fn main() {
    let x = Rc::new(RefCell::new(String::from("Hello")));
    let y = x.borrow();
    let z = test(&x);
    println!("{z}");
}

fn test(x: &Rc<RefCell<String>>) -> Ref<String> {
    x.borrow()
}

fn return_ref_to_inner_option(x: &Option<String>) -> Option<&String> {
    match x {
        Some(y) => Some(y),
        None => None,
    }
}

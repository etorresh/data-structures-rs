use std::rc::Rc;
fn main() {
    let mut a = Rc::new(5);
    let b = Rc::clone(&a);

    a = Rc::new(10);

    println!("a = {:?}", a);
    println!("b = {:?}", b);
}

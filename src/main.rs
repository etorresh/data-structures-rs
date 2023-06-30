use std::rc::Rc;

struct MyStruct {
    field: u32,
}

struct human {
    name: String,
    sibling: Option<Rc<human>>,
}

fn main() {
    let a = Rc::new(human {
        name: String::from("Juan"),
        sibling: None,
    });
    let b = human {
        name: String::from("Juana"),
        sibling: Some(Rc::clone(&a)),
    };
    let c = human {
        name: String::from("Adriana"),
        sibling: Some(Rc::clone(&a)),
    };
}

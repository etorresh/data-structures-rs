struct MyStruct {
    field: u32,
}

struct human {
    name: String,
    sibling: Option<Box<human>>,
}

fn main() {
    let x = Box::new(human {
        name: String::from("Juan"),
        sibling: None,
    });
    let y = Box::new(human {
        name: String::from("Adriana"),
        sibling: Some(x),
    });
    let mut z = Box::new(human {
        name: String::from("Mayorga"),
        sibling: Some(y),
    });
    // y owns x
    let mut z_ref = &mut z;
    z_ref.name = String::from("yes");
    //let sibling_ref = &mut z.sibling;
    z_ref.name = String::new();

    let mut my_struct = MyStruct { field: 42 };
    let struct_ref = &mut my_struct;
    let field_ref = &mut struct_ref.field; // Error: cannot have both `struct_ref` and `field_ref` as mutable references

    *field_ref = 10;
    // conclusion: it's impossible to have a mutable reference to a current node and a previous node without using smart pointers or unsafe.
}

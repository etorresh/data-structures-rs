// Singly linked ist
fn main() {
    // let x = Node {
    //     data: 5,
    //     next: None,
    // };
    // let y = Node {
    //     data: 10,
    //     next: Some(Box::new(x)),
    // };
    // let z = Node {
    //     data: 15,
    //     next: Some(Box::new(y)),
    // };

    // let head = z;
    // if head.next.unwrap().next.unwrap().data == 5 {
    //     println!("I can see the last value")
    // }
    let x: LinkedList<i32> = LinkedList {
        head: None,
        counter: 0,
    };
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    counter: usize,
}

impl<T> LinkedList<T> {
    fn add_first(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.counter += 1;
    }

    fn add_last(&mut self, data: T) {
        let current = &self.head;
        match current {
            Some(_) => {}
            None => self.add_first(data),
        }
        // let new_node = Box::new(Node { data, next: None });
    }
}

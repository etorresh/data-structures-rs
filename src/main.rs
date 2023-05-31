// Singly linked ist
fn main() {
    let mut x: LinkedList<i32> = LinkedList::new();
    x.add_last(5);
    x.add_last(10);
    println!("{}", x.head.unwrap().next.unwrap().data);
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
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            counter: 0,
        }
    }
    fn add_first(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);

        self.counter += 1;
    }

    fn add_last(&mut self, data: T) {
        let mut current = &mut self.head;
        loop {
            match current {
                Some(node) => current = &mut node.next,
                None => break,
            }
        }
        let new_node = Box::new(Node { data, next: None });
        *current = Some(new_node);

        self.counter += 1;
    }

    fn remove_first(&mut self) {
        self.head = self.head.take().and_then(|node| node.next);
        self.counter -= 1;
    }
    fn remove_last(&mut self) {
        let mut current = &mut self.head;
        loop {
            match current {
                Some(node) => current = &mut node.next,
                None => break,
            }
        }
        self.counter -= 1;
    }
    fn remove() {}
    fn find() {}
    fn peek() {}
}

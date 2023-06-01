// Singly linked ist
fn main() {
    let mut x: LinkedList<i32> = LinkedList::new();
    x.remove_first();
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
        if self.head.is_none() {
            return;
        }
        self.head = self.head.take().and_then(|node| node.next);
        self.counter -= 1;
    }

    fn remove_last(&mut self) {
        // Look ahead solution: traverse looking two nodes ahead to see if we're at the second to last node.
        // Count and cut: traverse once and count the values. Traverse again and stop at count - 1
        // Best option: use current and previous, but how the fuck do we handle two mutable references?

        /* */
        if self.head.is_none() {
            return;
        }
        let mut current = &self.head;
        let mut node_count = 0;
        loop {
            match current {
                Some(node) => {
                    current = &node.next;
                    node_count += 1;
                }
                None => break,
            }
        }
        let mut current = &mut self.head;
        for _ in 0..node_count - 1 {
            match current {
                Some(node) => current = &mut node.next,
                None => break,
            }
        }
        *current = current.take().and_then(|node| node.next);
        self.counter -= 1;
    }
    fn remove() {}
    fn find() {}
    fn peek() {}
}

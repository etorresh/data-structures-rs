// Singly linked ist
fn main() {
    let mut x: LinkedList<i32> = LinkedList::new();
    x.add_first(5);
    x.add_first(10);
    x.remove_last();
    println!("{}", x.head.unwrap().data);
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
        // Look Ahead Strategy: Traverse through the list, always checking two nodes ahead to determine if the current node is the penultimate node.
        // Count and Cut Strategy: Traverse through the list once while counting the nodes. Traverse again and stop at count - 1, essentially cutting the last node.
        // Optimal Strategy: Maintain two pointers, current and previous, but handling two mutable references simultaneously requires careful consideration in Rust due to the borrow checker.

        /* COUNT AND CUT
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
        */

        /* LOOK AHEAD */
        let mut current = &mut self.head;
        while let Some(node) = current {
            if node
                .next
                .as_ref()
                .and_then(|next_node| next_node.next.as_ref())
                .is_some()
            {
                current = &mut node.next;
            } else {
                node.next = None;
                self.counter -= 1;
                break;
            }
        }
    }
    fn remove() {}
    fn find() {}
    fn peek() {}
}

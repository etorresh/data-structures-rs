// Singly linked ist
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    counter: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            counter: 0,
        }
    }
    pub fn add_first(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);

        self.counter += 1;
    }

    pub fn add_last(&mut self, data: T) {
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

    pub fn remove_first(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                self.counter -= 1;
                Some(node.data)
            }
            None => None,
        }
    }

    pub fn remove_last(&mut self) {
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
        // Start at the head of the list
        let mut current = &mut self.head;

        // Check case where there is a single element in the structure
        if let Some(node) = current {
            // If the node doesn't have a next node, remove it and return early.
            if node.next.is_none() {
                *current = None;
                self.counter -= 1;
                return;
            }
        } else {
            // If the first node is None, then the list is empty. Return early.
            return;
        }

        // Traverse the list looking two nodes ahead
        while let Some(node) = current {
            // If two nodes ahead exist, move our reference one node ahead
            if node
                .next
                .as_ref()
                .and_then(|next_node| next_node.next.as_ref())
                .is_some()
            {
                current = &mut node.next;
            } else {
                // If there is no two nodes ahead, remove the last node.
                node.next = None;
                self.counter -= 1;
                break;
            }
        }

        /* Optimal strategy */
    }
    pub fn remove() {}
    pub fn find() {}
    pub fn peek() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_first() {
        let mut x = LinkedList::new();
        x.add_first(10);
        x.add_first(5);
        assert_eq!(x.head.unwrap().data, 5);
    }

    #[test]
    fn add_last() {
        let mut x = LinkedList::new();
        x.add_last(5);
        x.add_last(10);
        assert_eq!(x.head.unwrap().data, 5);
    }

    #[test]
    fn add_last_to_existing_list() {
        let mut x = LinkedList::new();
        x.add_first(10);
        x.add_first(5);
        x.add_last(15);
        assert_eq!(x.head.unwrap().next.unwrap().next.unwrap().data, 15);
    }

    #[test]
    fn remove_first_empty_list() {
        let mut x: LinkedList<i32> = LinkedList::new();
        x.remove_first();
        assert_eq!(x.counter, 0);
    }

    #[test]
    fn remove_last_empty_list() {
        let mut x: LinkedList<i32> = LinkedList::new();
        x.remove_last();
        assert_eq!(x.counter, 0);
    }

    #[test]
    fn remove_first_single_element() {
        let mut x = LinkedList::new();
        x.add_first(5);
        x.remove_first();
        assert_eq!(x.counter, 0);
        assert!(x.head.is_none());
    }

    #[test]
    fn remove_last_single_element() {
        let mut x = LinkedList::new();
        x.add_first(5);
        x.remove_last();
        assert_eq!(x.counter, 0);
        assert!(x.head.is_none());
    }

    #[test]
    fn remove_first_two_elements() {
        let mut x = LinkedList::new();
        x.add_first(5);
        x.add_first(10);
        x.remove_first();
        assert_eq!(x.counter, 1);
        assert_eq!(x.head.as_ref().unwrap().data, 5);
        assert!(x.head.and_then(|next_node| next_node.next).is_none());
    }

    #[test]
    fn remove_last_two_elements() {
        let mut x = LinkedList::new();
        x.add_first(5);
        x.add_first(10);
        x.remove_last();
        assert_eq!(x.counter, 1);
        assert_eq!(x.head.as_ref().unwrap().data, 10);
        assert!(x.head.and_then(|next_node| next_node.next).is_none());
    }
}

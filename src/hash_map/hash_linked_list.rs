/**
 * Singly Linked List modified to work wih a Hash Map
 */

// Iterator strucs
pub struct IntoIter<T: PartialEq>(LinkedList<T>);
pub struct IterMut<'a, T: PartialEq> {
    current_link: &'a mut Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct LinkedList<T: PartialEq> {
    head: Link<T>,
    counter: usize,
}

impl<T: PartialEq> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        let head = None;
        LinkedList { head, counter: 0 }
    }
    pub fn add_first(&mut self, data: T) {
        let new_node = Some(Box::new(Node {
            data,
            next: self.head.take(),
        }));

        self.head = new_node;
        self.counter += 1;
    }

    pub fn add_last(&mut self, data: T) {
        let new_tail_node = Some(Box::new(Node { data, next: None }));

        let mut current_link = &mut self.head;
        while let Some(node) = current_link {
            current_link = &mut node.next;
        }
        *current_link = new_tail_node;

        self.counter += 1;
    }

    pub fn remove_first(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.counter -= 1;
            node.data
        })
    }

    pub fn remove_last(&mut self) {
        if self.head.is_none() {
            return;
        }

        if self.counter == 1 {
            self.remove_first();
            return;
        }

        let mut current_link = &mut self.head;
        for _ in 0..(self.counter - 2) {
            if let Some(node) = current_link {
                current_link = &mut node.next;
            } else {
                panic!("Error remove_last: list count is inconsistent with actual number of nodes. 'self.counter' is bigger than the actual number of nodes");
            }
        }

        if let Some(node) = current_link {
            node.next = None;
        }
        self.counter -= 1;
    }

    pub fn reverse(&mut self) {
        let mut prev_link = None;
        let mut current_link = self.head.take();

        while let Some(mut current_node) = current_link {
            let next_link = current_node.next.take();
            current_node.next = prev_link;
            prev_link = Some(current_node);
            current_link = next_link;
        }
        self.head = prev_link;
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn contains(&self, x: &T) -> bool {
        let mut contains = false;

        let mut current_link = &self.head;
        while let Some(current_node) = current_link {
            if current_node.data == x {
                contains = true;
                break;
            }
            current_link = &current_node.next;
        }

        contains
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            current_link: &mut self.head,
        }
    }
}

impl<T: PartialEq> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.remove_first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_first() {
        let mut x = LinkedList::new();
        x.add_first(10);
        x.add_first(5);
        assert_eq!(x.head.as_ref().unwrap().data, 5);
    }

    #[test]
    fn add_last() {
        let mut x = LinkedList::new();
        x.add_last(5);
        x.add_last(10);
        assert_eq!(x.head.as_ref().unwrap().data, 5);
    }

    #[test]
    fn add_last_no_elements() {
        let mut x = LinkedList::new();
        x.add_last(5);
        assert_eq!(x.head.as_ref().unwrap().data, 5);
    }

    #[test]
    fn add_last_to_existing_list() {
        let mut x = LinkedList::new();
        x.add_first(10);
        x.add_first(5);
        x.add_last(15);
        assert_eq!(
            x.head
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .data,
            15
        );
    }

    #[test]
    fn add_last_multiple_elements() {
        let mut x = LinkedList::new();
        x.add_last(5);
        x.add_last(10);
        x.add_last(15);
        assert_eq!(x.head.as_ref().unwrap().data, 5);
    }

    #[test]
    fn add_last_check_counter() {
        let mut x = LinkedList::new();
        x.add_last(5);
        x.add_last(10);
        x.add_last(15);
        assert_eq!(x.counter, 3);
    }

    #[test]
    fn add_last_single_element() {
        let mut x = LinkedList::new();
        x.add_last(5);
        assert_eq!(x.head.as_ref().unwrap().data, 5);
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
    }

    #[test]
    fn remove_last_two_elements() {
        let mut x = LinkedList::new();
        x.add_first(5);
        x.add_first(10);
        x.remove_last();
        assert_eq!(x.counter, 1);
        assert_eq!(x.remove_first().unwrap(), 10);
        assert!(x.head.is_none());
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedList::new();
        list.add_first(1);
        list.add_first(2);
        list.add_first(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn reverse() {
        let mut list = LinkedList::new();
        list.add_first(3);
        list.add_first(2);
        list.add_first(1);

        list.reverse();

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn counter() {
        let mut list = LinkedList::new();
        for x in 1..100 {
            list.add_first(x);
        }
        for _ in 1..100 {
            list.remove_last();
        }
        assert_eq!(list.counter, 0);
    }

    #[test]
    fn remove_first_overflow() {
        let mut list = LinkedList::new();
        list.add_first(1);
        list.remove_first();
        list.remove_first();
    }
}

use std::borrow::BorrowMut;
/**
 * Singly linked List with a tail pointer
 * Makes add_last o(1)
 */
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct LinkedListSinglyTail<T> {
    head: Link<T>,
    tail: Link<T>,
    counter: usize,
}

impl<T> LinkedListSinglyTail<T> {
    pub fn new() -> LinkedListSinglyTail<T> {
        let head = None;
        let tail = None;
        LinkedListSinglyTail {
            head,
            tail,
            counter: 0,
        }
    }
    pub fn add_first(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: self.head.as_ref().map(|node| Rc::clone(node)),
        }));

        if self.head.is_none() {
            self.tail = Some(Rc::clone(&new_node));
        }

        self.head = Some(new_node);
        self.counter += 1;
    }

    pub fn add_last(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node { data, next: None }));

        match &self.tail {
            Some(tail) => {
                tail.borrow_mut().next = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::clone(&new_node));
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }

        self.counter += 1;
    }

    pub fn remove_first(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.borrow_mut().next.take();
            self.counter -= 1;
            if self.head.is_none() {
                self.tail = None;
            }
            // Continue here
            let data = std::mem::take(node.borrow_mut().data);
            data
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

        // Move to the second last node in the list
        let mut current_link = &mut self.head;
        for _ in 0..(self.counter - 2) {
            current_link = &mut current_link.as_mut().expect("Error remove_last: list count is inconsistent with actual number of nodes. 'self.counter' is bigger than the actual number of nodes").borrow_mut().next;
        }

        current_link
            .expect("Error remove last: at this point current_link should always be valid")
            .borrow_mut()
            .next = None;
        self.counter -= 1;
    }

    pub fn reverse(&mut self) {
        let mut prev_link = None;
        let mut current_link = self.head.take();

        while let Some(mut current_node) = current_link {
            let next_link = current_node.borrow_mut().next.take();
            current_node.borrow_mut().next = prev_link;
            prev_link = Some(current_node);
            current_link = next_link;
        }
        self.head = prev_link;
    }
}

pub struct IntoIter<T>(LinkedListSinglyTail<T>);
impl<T> LinkedListSinglyTail<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
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
        let mut x = LinkedListSinglyTail::new();
        x.add_first(10);
        x.add_first(5);
        assert_eq!(x.head.unwrap().borrow().data, 5);
    }

    #[test]
    fn add_last() {
        let mut x = LinkedListSinglyTail::new();
        x.add_last(5);
        x.add_last(10);
        assert_eq!(x.head.unwrap().borrow().data, 5);
    }

    #[test]
    fn add_last_no_elements() {
        let mut x = LinkedListSinglyTail::new();
        x.add_last(5);
        assert_eq!(x.head.unwrap().borrow().data, 5);
    }

    #[test]
    fn add_last_to_existing_list() {
        let mut x = LinkedListSinglyTail::new();
        x.add_first(10);
        x.add_first(5);
        x.add_last(15);
        assert_eq!(
            x.head
                .unwrap()
                .borrow()
                .next
                .unwrap()
                .borrow()
                .next
                .unwrap()
                .borrow()
                .data,
            15
        );
    }

    #[test]
    fn add_last_multiple_elements() {
        let mut x = LinkedListSinglyTail::new();
        x.add_last(5);
        x.add_last(10);
        x.add_last(15);
        assert_eq!(x.head.unwrap().borrow().data, 5);
        assert_eq!(x.tail.unwrap().borrow().data, 15);
    }

    #[test]
    fn add_last_check_counter() {
        let mut x = LinkedListSinglyTail::new();
        x.add_last(5);
        x.add_last(10);
        x.add_last(15);
        assert_eq!(x.counter, 3);
    }

    #[test]
    fn add_last_single_element() {
        let mut x = LinkedListSinglyTail::new();
        x.add_last(5);
        assert_eq!(x.head.unwrap().borrow().data, 5);
        assert_eq!(x.tail.unwrap().borrow().data, 5);
    }

    #[test]
    fn remove_first_empty_list() {
        let mut x: LinkedListSinglyTail<i32> = LinkedListSinglyTail::new();
        x.remove_first();
        assert_eq!(x.counter, 0);
    }

    #[test]
    fn remove_last_empty_list() {
        let mut x: LinkedListSinglyTail<i32> = LinkedListSinglyTail::new();
        x.remove_last();
        assert_eq!(x.counter, 0);
    }

    #[test]
    fn remove_first_single_element() {
        let mut x = LinkedListSinglyTail::new();
        x.add_first(5);
        x.remove_first();
        assert_eq!(x.counter, 0);
        assert!(x.head.is_none());
    }

    #[test]
    fn remove_last_single_element() {
        let mut x = LinkedListSinglyTail::new();
        x.add_first(5);
        x.remove_last();
        assert_eq!(x.counter, 0);
        assert!(x.head.is_none());
    }

    #[test]
    fn remove_first_two_elements() {
        let mut x = LinkedListSinglyTail::new();
        x.add_first(5);
        x.add_first(10);
        x.remove_first();
        assert_eq!(x.counter, 1);
        assert_eq!(x.head.unwrap().borrow().data, 5);
    }

    #[test]
    fn remove_last_two_elements() {
        let mut x = LinkedListSinglyTail::new();
        x.add_first(5);
        x.add_first(10);
        x.remove_last();
        assert_eq!(x.counter, 1);
        assert_eq!(x.remove_first().unwrap(), 10);
        assert!(x.head.is_none());
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedListSinglyTail::new();
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
        let mut list = LinkedListSinglyTail::new();
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
        let mut list = LinkedListSinglyTail::new();
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
        let mut list = LinkedListSinglyTail::new();
        list.add_first(1);
        list.remove_first();
        list.remove_first();
    }
}

/**
 * Doubly Linked List.
 */
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

type Link<T> = Rc<RefCell<Option<Node<T>>>>;
struct Node<T> {
    data: T,
    next: Link<T>,
    previous: Weak<RefCell<Option<Node<T>>>>,
}

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    counter: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        let head = Rc::new(RefCell::new(None));
        let tail = Rc::clone(&head);
        DoublyLinkedList {
            head,
            tail,
            counter: 0,
        }
    }
    // Fix: update the previous first node so it points to the new node.
    pub fn add_first(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Some(Node {
            data,
            next: Rc::clone(&self.head),
            previous: Weak::new(),
        })));

        match *self.head.borrow_mut() {
            Some(ref mut node) => node.previous = Rc::downgrade(&new_node),
            // // If this is the first element added to the list then also set the tail to be equal to this new node
            None => self.tail = Rc::clone(&new_node),
        }

        self.head = new_node;
        self.counter += 1;
    }

    pub fn add_last(&mut self, data: T) {
        let mut current_tail_node = self.tail.take();
        if current_tail_node.is_none() {
            self.add_first(data);
            return;
        }

        let new_tail_node = Rc::new(RefCell::new(Some(Node {
            data,
            next: Rc::new(RefCell::new(None)),
            previous: Rc::downgrade(&self.tail),
        })));

        if let Some(ref mut node) = current_tail_node {
            node.next = Rc::clone(&new_tail_node);
        }

        *self.tail.borrow_mut() = current_tail_node;
        self.tail = Rc::clone(&new_tail_node);
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
        if self.counter == 0 {
            return;
        }

        if self.counter == 1 {
            self.remove_first();
            return;
        }

        let current_tail = self.tail.take();
        let previous_link_option = current_tail.and_then(|tail_node| tail_node.previous.upgrade());
        if previous_link_option.is_some() {
            println!("is some");
        } else {
            println!("is none");
        }
        match previous_link_option {
            Some(previous_link) => {
                println!("inside previous_link some");
                self.tail = Rc::clone(&previous_link);
            }
            None => {
                println!("returning");
                return;
            }
        };
        self.counter -= 1;
    }

    pub fn reverse(&mut self) {
        let mut prev_link = None;
        let mut current_link = self.head.take();

        while let Some(mut current_node) = current_link {
            let next_link = current_node.next.take();
            current_node.next = Rc::new(RefCell::new(prev_link));
            prev_link = Some(current_node);
            current_link = next_link;
        }
        self.head = Rc::new(RefCell::new(prev_link));
    }
}

pub struct IntoIter<T>(DoublyLinkedList<T>);
impl<T> DoublyLinkedList<T> {
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
        let mut x = DoublyLinkedList::new();
        x.add_first(10);
        x.add_first(5);
        assert_eq!(x.head.borrow().as_ref().unwrap().data, 5);
    }

    #[test]
    fn add_last() {
        let mut x = DoublyLinkedList::new();
        x.add_last(5);
        x.add_last(10);
        assert_eq!(x.head.borrow().as_ref().unwrap().data, 5);
    }

    #[test]
    fn add_last_no_elements() {
        let mut x = DoublyLinkedList::new();
        x.add_last(5);
        assert_eq!(x.head.borrow().as_ref().unwrap().data, 5);
    }

    #[test]
    fn add_last_to_existing_list() {
        let mut x = DoublyLinkedList::new();
        x.add_first(10);
        x.add_first(5);
        x.add_last(15);
        assert_eq!(
            x.head
                .borrow()
                .as_ref()
                .unwrap()
                .next
                .borrow()
                .as_ref()
                .unwrap()
                .next
                .borrow()
                .as_ref()
                .unwrap()
                .data,
            15
        );
    }

    #[test]
    fn add_last_multiple_elements() {
        let mut x = DoublyLinkedList::new();
        x.add_last(5);
        x.add_last(10);
        x.add_last(15);
        assert_eq!(x.head.borrow().as_ref().unwrap().data, 5);
        assert_eq!(x.tail.borrow().as_ref().unwrap().data, 15);
    }

    #[test]
    fn add_last_check_counter() {
        let mut x = DoublyLinkedList::new();
        x.add_last(5);
        x.add_last(10);
        x.add_last(15);
        assert_eq!(x.counter, 3);
    }

    #[test]
    fn add_last_single_element() {
        let mut x = DoublyLinkedList::new();
        x.add_last(5);
        assert_eq!(x.head.borrow().as_ref().unwrap().data, 5);
        assert_eq!(x.tail.borrow().as_ref().unwrap().data, 5);
    }

    #[test]
    fn remove_first_empty_list() {
        let mut x: DoublyLinkedList<i32> = DoublyLinkedList::new();
        x.remove_first();
        assert_eq!(x.counter, 0);
    }

    #[test]
    fn remove_last_empty_list() {
        let mut x: DoublyLinkedList<i32> = DoublyLinkedList::new();
        x.remove_last();
        assert_eq!(x.counter, 0);
    }

    #[test]
    fn remove_first_single_element() {
        let mut x = DoublyLinkedList::new();
        x.add_first(5);
        x.remove_first();
        assert_eq!(x.counter, 0);
        assert!(x.head.borrow().is_none());
    }

    #[test]
    fn remove_last_single_element() {
        let mut x = DoublyLinkedList::new();
        x.add_first(5);
        x.remove_last();
        assert_eq!(x.counter, 0);
        assert!(x.head.borrow().is_none());
    }

    #[test]
    fn remove_first_two_elements() {
        let mut x = DoublyLinkedList::new();
        x.add_first(5);
        x.add_first(10);
        x.remove_first();
        assert_eq!(x.counter, 1);
        assert_eq!(x.head.borrow().as_ref().unwrap().data, 5);
    }

    #[test]
    fn remove_last_two_elements() {
        let mut x = DoublyLinkedList::new();
        x.add_first(5);
        x.add_first(10);
        x.remove_last();
        assert_eq!(x.counter, 1);
        assert_eq!(x.remove_first().unwrap(), 10);
        assert!(x.head.borrow().is_none());
    }

    #[test]
    fn into_iter() {
        let mut list = DoublyLinkedList::new();
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
        let mut list = DoublyLinkedList::new();
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
        let mut list = DoublyLinkedList::new();
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
        let mut list = DoublyLinkedList::new();
        list.add_first(1);
        list.remove_first();
        list.remove_first();
    }
}

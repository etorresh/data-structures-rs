/**
 * Doubly Linked List.
 * Compared to my singly_linked_list implementation this one adds a tail pointer, and a pointer to the previous node.
 */
// I had std::borrow::BorrowMut which was shadowing the method of the same name in RefCell.
// I'm learning Rust so I thought the error was on my logic, my brain almost fucking fried.
// https://github.com/rust-lang/rust/issues/39232 a PR was added a few months before this that makes the type check warn you lol

// TODOs for DoublyLinkedList:
// 1. Handle `previous` pointers in `reverse` and `remove` methods.
// 2. Implement the `Drop` trait for memory cleanup.
// 3. Refactor to eliminate repetitive patterns.
// 4. Add comments explaining core logic.
// 5. Implement a reverse iterator.
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

type Link<T> = Rc<RefCell<Option<Node<T>>>>;
struct Node<T> {
    data: T,
    next: Link<T>,
    previous: Weak<RefCell<Option<Node<T>>>>,
}

pub struct DoublyLinkedList<T: PartialEq> {
    head: Link<T>,
    tail: Link<T>,
    counter: usize,
}

impl<T> DoublyLinkedList<T>
where
    T: PartialEq,
{
    pub fn new() -> DoublyLinkedList<T> {
        let head = Rc::new(RefCell::new(None));
        let tail = Rc::clone(&head);
        DoublyLinkedList {
            head,
            tail,
            counter: 0,
        }
    }
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
        if self.counter == 0 {
            self.add_first(data);
            return;
        }
        let mut current_tail_node = self.tail.take();

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
        // Handles empty list and list with a single element
        if self.counter < 2 {
            self.remove_first();
            return;
        }

        self.tail = self
            .tail
            .take()
            .and_then(|tail_node| tail_node.previous.upgrade())
            .expect("Error: previous node of tail node is None");

        self.counter -= 1;
    }

    // To do: this doesn't handle previous pointers
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

    // TO DO: this is not handling previous pointers
    pub fn remove(&mut self, data: T) {
        let mut current = self.head.clone();
        let mut prev = Rc::new(RefCell::new(None::<Node<T>>));

        while current.borrow().is_some() {
            let next;
            {
                let borrowed = current.borrow();
                let node = borrowed.as_ref().unwrap();
                if node.data == data {
                    if Rc::ptr_eq(&self.head, &current) {
                        self.head = Rc::clone(&node.next);

                        if let Some(new_head) = self.head.borrow_mut().as_mut() {
                            new_head.previous = Weak::new();
                        }
                    } else {
                        prev.borrow_mut().as_mut().unwrap().next = Rc::clone(&node.next);

                        if let Some(next_node) = node.next.borrow_mut().as_mut() {
                            next_node.previous = Rc::downgrade(&prev);
                        }
                    }
                    self.counter -= 1;

                    return;
                }
                next = current.borrow().as_ref().unwrap().next.clone();
            }
            prev = current.clone();
            current = next;
        }
    }
}

pub struct IntoIter<T>(DoublyLinkedList<T>)
where
    T: PartialEq;
impl<T: PartialEq> DoublyLinkedList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
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

    #[test]
    fn remove_middle() {
        let mut list = DoublyLinkedList::new();
        list.add_first(10);
        list.add_first(9);
        list.add_first(8);
        list.remove(9);
        assert_eq!(list.counter, 2);
        assert_eq!(list.remove_first().unwrap(), 8);
        assert_eq!(list.remove_first().unwrap(), 10);
    }

    #[test]
    fn remove_head() {
        let mut list = DoublyLinkedList::new();
        list.add_first(10);
        list.add_first(9);
        list.add_first(8);
        list.remove(8);
        assert_eq!(list.counter, 2);
        assert_eq!(list.remove_first().unwrap(), 9);
    }

    #[test]
    fn remove_empty_list() {
        let mut list = DoublyLinkedList::new();
        list.remove(2);
    }

    #[test]
    fn remove_tail() {
        let mut list = DoublyLinkedList::new();
        list.add_first(10);
        list.add_first(9);
        list.add_first(8);
        list.remove(10);
        assert_eq!(list.counter, 2);
        assert_eq!(list.remove_first().unwrap(), 8);
        assert_eq!(list.remove_first().unwrap(), 9);
        assert!(list.remove_first().is_none());
    }

    #[test]
    fn remove_non_existent() {
        let mut list = DoublyLinkedList::new();
        list.add_first(10);
        list.add_first(9);
        list.add_first(8);
        list.remove(7);
        assert_eq!(list.counter, 3);
        assert_eq!(list.remove_first().unwrap(), 8);
        assert_eq!(list.remove_first().unwrap(), 9);
        assert_eq!(list.remove_first().unwrap(), 10);
    }

    #[test]
    fn remove_repeated_element() {
        let mut list = DoublyLinkedList::new();
        list.add_first(10);
        list.add_first(9);
        list.add_first(9);
        list.add_first(8);
        list.remove(9);
        assert_eq!(list.counter, 3);
        assert_eq!(list.remove_first().unwrap(), 8);
        assert_eq!(list.remove_first().unwrap(), 9);
        assert_eq!(list.remove_first().unwrap(), 10);
    }

    #[test]
    fn remove_from_single_element_list() {
        let mut list = DoublyLinkedList::new();
        list.add_first(8);
        list.remove(8);
        assert_eq!(list.counter, 0);
        assert!(list.remove_first().is_none());
    }
}

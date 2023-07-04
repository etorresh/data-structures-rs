use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Rc<RefCell<Option<Node<T>>>>;
struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
    counter: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: Rc::new(RefCell::new(None)),
            counter: 0,
        }
    }
    pub fn add_first(&mut self, data: T) {
        let new_node = Node {
            data,
            next: Rc::clone(&self.head),
        };
        self.head = Rc::new(RefCell::new(Some(new_node)));

        self.counter += 1;
    }

    pub fn add_last(&mut self, data: T) {
        let mut current = &mut self.head;
        loop {
            match current.borrow().as_ref() {
                Some(node) => current = &mut node.next,
                None => break,
            }
        }
        let new_node = Node {
            data,
            next: Rc::new(RefCell::new(None)),
        };
        *current.borrow_mut() = Rc::new(RefCell::new(Some(new_node)));

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
        // Look Ahead Strategy: Traverse through the list, always checking two nodes ahead to determine if the current node is the penultimate node.
        // Count and Cut Strategy: Traverse through the list once while counting the nodes. Traverse again and stop at count - 1, essentially cutting the last node.
        // Optimal Strategy: Maintain two pointers, current and previous, but I don't know how to handle two mutable references simultaneously. Rc and RefCell? I need to read

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

        // LOOK AHEAD
        // Start at the head of the list
        let mut current = &mut self.head;

        // Check case where there is a single element in the structure
        if let Some(node) = current.borrow().as_ref() {
            // If the node doesn't have a next node, remove it and return early.
            if node.next.borrow().is_none() {
                *current.borrow_mut() = Rc::new(RefCell::new(None));
                self.counter -= 1;
                return;
            }
        } else {
            // If the first node is None, then the list is empty. Return early.
            return;
        }

        // Traverse the list looking two nodes ahead
        while let Some(ref mut node) = current.borrow().as_ref() {
            // If two nodes ahead exist, move our reference one node ahead
            if node
                .next
                .borrow()
                .and_then(|next_node| *next_node.next.borrow())
                .is_some()
            {
                current = &mut node.next;
            } else {
                // If there is no two nodes ahead, remove the last node.
                node.next = Rc::new(RefCell::new(None));
                self.counter -= 1;
                break;
            }
        }

        /* Optimal strategy */
    }
    pub fn remove() {}
    pub fn find() {}
    // pub fn peek(&self) -> Option<&T> {
    //     self.head.borrow().map(|node| &node.data)
    // }

    // pub fn peek_mut(&mut self) -> Option<&mut T> {
    //     self.head.borrow_mut().map(|ref mut node| &mut node.data)
    // }

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

pub struct IntoIter<T>(LinkedList<T>);
impl<T> LinkedList<T> {
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

// pub struct Iter<'a, T> {
//     next: Option<&'a Node<T>>,
// }

// impl<T> LinkedList<T> {
//     pub fn iter(&self) -> Iter<T> {
//         Iter {
//             // equivalent to self.head.as_ref().map(|node| &**node)
//             next: self.head.as_deref(),
//         }
//     }
// }

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.map(|node| {
//             // self.next = node.next.as_ref().map(|node| &**node)
//             self.next = node.next.as_deref();
//             &node.data
//         })
//     }
// }

// pub struct IterMut<'a, T> {
//     next: Option<&'a mut Node<T>>,
// }

// impl<T> LinkedList<T> {
//     pub fn iter_mut(&mut self) -> IterMut<'_, T> {
//         IterMut {
//             next: self.head.as_deref_mut(),
//         }
//     }
// }

// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.take().map(|node| {
//             self.next = node.next.as_deref_mut();
//             &mut node.data
//         })
//     }
// }

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

    #[test]
    fn peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.add_first(1);
        list.add_first(2);
        list.add_first(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.remove_first(), Some(42));
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
    fn iter() {
        let mut list = LinkedList::new();
        list.add_first(1);
        list.add_first(2);
        list.add_first(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = LinkedList::new();
        list.add_first(1);
        list.add_first(2);
        list.add_first(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

    #[test]
    fn reverse() {
        let mut list = LinkedList::new();
        list.add_first(3);
        list.add_first(2);
        list.add_first(1);

        list.reverse();

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn counter() {
        let mut list = LinkedList::new();
        for x in 1..100 {
            list.add_first(x);
        }
        for x in 1..100 {
            list.remove_last();
        }
        assert_eq!(list.counter, 0);
    }
}

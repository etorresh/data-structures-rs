use std::thread::current;

/**
 * Singly Linked List modified to work wih a Hash Map
 */

// Iterator strucs
pub struct IntoIter<K: PartialEq, V>(HashLinkedList<K, V>);
pub struct IterMut<'a, K: PartialEq, V> {
    current_link: &'a mut Link<K, V>,
}

type Link<K: PartialEq, V> = Option<Box<Node<K, V>>>;
struct Node<K: PartialEq, V> {
    key: K,
    value: V,
    next: Link<K, V>,
}

pub struct HashLinkedList<K: PartialEq, V> {
    head: Link<K, V>,
    counter: usize,
}

impl<K: PartialEq, V> HashLinkedList<K, V> {
    pub fn new() -> HashLinkedList<K, V> {
        let head = None;
        HashLinkedList { head, counter: 0 }
    }
    pub fn add_first(&mut self, key: K, value: V) {
        let new_node = Some(Box::new(Node {
            key,
            value,
            next: self.head.take(),
        }));

        self.head = new_node;
        self.counter += 1;
    }

    pub fn add_last(&mut self, key: K, value: V) {
        let new_tail_node = Some(Box::new(Node {
            key,
            value,
            next: None,
        }));

        let mut current_link = &mut self.head;
        while let Some(node) = current_link {
            current_link = &mut node.next;
        }
        *current_link = new_tail_node;

        self.counter += 1;
    }

    pub fn remove_first(&mut self) -> Option<V> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.counter -= 1;
            node.value
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

    pub fn peek(&self) -> Option<&V> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut current_link = &mut self.head;

        while let Some(node) = current_link {
            if node.key == key {
                let old_value = std::mem::replace(&mut node.value, value);
                return Some(old_value);
            }
            current_link = &mut node.next;
        }

        let new_node = Some(Box::new(Node {
            key,
            value: value,
            next: None,
        }));

        *current_link = new_node;
        self.counter += 1;
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut current_link = &self.head;

        while let Some(node) = current_link {
            if &node.key == key {
                return Some(&node.value);
            }
            current_link = &node.next;
        }
        None
    }

    /*
    Implementing remove helped me understand the borrow checker differences between while let Some(x) and loop+match
    Main points
    - When using while let, y remains borrowed for the entirety of the loop's body, preventing other operations on it within that scope.
    - On the other hand, match just takes a temporary borrow to check patterns and bind values. Once inside a successful arm, the borrow is released.
    - In a successful match arm, the variables bound by the pattern (like x in Some(x)) directly access the inner values, and the original matched value
    (y in this case) is left unborrowed and free for other uses.
    */
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut current_link = &mut self.head;
        loop {
            match current_link {
                None => return None,
                Some(node) if &node.key == key => {
                    let mut removed_node = current_link.take();
                    *current_link = removed_node.as_mut().and_then(|node| node.next.take());
                    self.counter -= 1;
                    return removed_node.map(|node| node.value);
                }
                Some(node) => {
                    current_link = &mut node.next;
                }
            }
        }
    }

    pub fn into_iter(self) -> IntoIter<K, V> {
        IntoIter(self)
    }
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            current_link: &mut self.head,
        }
    }
}

impl<K: PartialEq, V> Iterator for IntoIter<K, V> {
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.remove_first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_first() {
        let mut x = HashLinkedList::new();
        x.add_first("", 10);
        x.add_first("", 5);
        assert_eq!(x.head.as_ref().unwrap().value, 5);
    }

    #[test]
    fn add_last() {
        let mut x = HashLinkedList::new();
        x.add_last("", 5);
        x.add_last("", 10);
        assert_eq!(x.head.as_ref().unwrap().value, 5);
    }

    #[test]
    fn add_last_no_elements() {
        let mut x = HashLinkedList::new();
        x.add_last("", 5);
        assert_eq!(x.head.as_ref().unwrap().value, 5);
    }

    #[test]
    fn add_last_to_existing_list() {
        let mut x = HashLinkedList::new();
        x.add_first("", 10);
        x.add_first("", 5);
        x.add_last("", 15);
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
                .value,
            15
        );
    }

    #[test]
    fn add_last_multiple_elements() {
        let mut x = HashLinkedList::new();
        x.add_last("", 5);
        x.add_last("", 10);
        x.add_last("", 15);
        assert_eq!(x.head.as_ref().unwrap().value, 5);
    }

    #[test]
    fn add_last_check_counter() {
        let mut x = HashLinkedList::new();
        x.add_last("", 5);
        x.add_last("", 10);
        x.add_last("", 15);
        assert_eq!(x.counter, 3);
    }

    #[test]
    fn add_last_single_element() {
        let mut x = HashLinkedList::new();
        x.add_last("", 5);
        assert_eq!(x.head.as_ref().unwrap().value, 5);
    }

    #[test]
    fn remove_first_empty_list() {
        let mut x: HashLinkedList<String, i32> = HashLinkedList::new();
        x.remove_first();
        assert_eq!(x.counter, 0);
    }

    #[test]
    fn remove_last_empty_list() {
        let mut x: HashLinkedList<String, i32> = HashLinkedList::new();
        x.remove_last();
        assert_eq!(x.counter, 0);
    }

    #[test]
    fn remove_first_single_element() {
        let mut x = HashLinkedList::new();
        x.add_first("", 5);
        x.remove_first();
        assert_eq!(x.counter, 0);
        assert!(x.head.is_none());
    }

    #[test]
    fn remove_last_single_element() {
        let mut x = HashLinkedList::new();
        x.add_first("", 5);
        x.remove_last();
        assert_eq!(x.counter, 0);
        assert!(x.head.is_none());
    }

    #[test]
    fn remove_first_two_elements() {
        let mut x = HashLinkedList::new();
        x.add_first("", 5);
        x.add_first("", 10);
        x.remove_first();
        assert_eq!(x.counter, 1);
        assert_eq!(x.head.as_ref().unwrap().value, 5);
    }

    #[test]
    fn remove_last_two_elements() {
        let mut x = HashLinkedList::new();
        x.add_first("", 5);
        x.add_first("", 10);
        x.remove_last();
        assert_eq!(x.counter, 1);
        assert_eq!(x.remove_first().unwrap(), 10);
        assert!(x.head.is_none());
    }

    #[test]
    fn into_iter() {
        let mut list = HashLinkedList::new();
        list.add_first("", 1);
        list.add_first("", 2);
        list.add_first("", 3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn reverse() {
        let mut list = HashLinkedList::new();
        list.add_first("", 3);
        list.add_first("", 2);
        list.add_first("", 1);

        list.reverse();

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn counter() {
        let mut list = HashLinkedList::new();
        for x in 1..100 {
            list.add_first("", x);
        }
        for _ in 1..100 {
            list.remove_last();
        }
        assert_eq!(list.counter, 0);
    }

    #[test]
    fn remove_first_overflow() {
        let mut list = HashLinkedList::new();
        list.add_first("", 1);
        list.remove_first();
        list.remove_first();
    }

    #[test]
    fn add_first_keys() {
        let mut x = HashLinkedList::new();
        x.add_first("Test1", 5);
        x.add_first("Test2", 10);
        x.remove_first();
        assert_eq!(x.counter, 1);
        assert_eq!(x.head.as_ref().unwrap().key, "Test1");
    }

    #[test]
    fn contains_key() {
        let mut x = HashLinkedList::new();
        x.add_last("Test1", 5);
        x.add_last("Test2", 10);
        x.add_last("Test3", 15);
        assert_eq!(x.head.as_ref().unwrap().value, 5);
        assert_eq!(x.head.as_ref().unwrap().key, "Test1")
    }

    #[test]
    fn remove_key_not_present() {
        let mut x = HashLinkedList::new();
        x.add_first("Test1", 5);
        x.add_first("Test2", 10);
        let result = x.remove(&"Test3");
        assert!(result.is_none());
        assert_eq!(x.counter, 2);
    }

    #[test]
    fn remove_only_element() {
        let mut x = HashLinkedList::new();
        x.add_first("Test1", 5);
        let removed_value = x.remove(&"Test1").unwrap();
        assert_eq!(removed_value, 5);
        assert_eq!(x.counter, 0);
        assert!(x.head.is_none());
    }

    #[test]
    fn remove_first_element() {
        let mut x = HashLinkedList::new();
        x.add_first("Test1", 5);
        x.add_first("Test2", 10);
        x.add_first("Test3", 15);
        let removed_value = x.remove(&"Test3").unwrap();
        assert_eq!(removed_value, 15);
        assert_eq!(x.counter, 2);
    }

    #[test]
    fn remove_middle_element() {
        let mut x = HashLinkedList::new();
        x.add_first("Test1", 5);
        x.add_first("Test2", 10);
        x.add_first("Test3", 15);
        let removed_value = x.remove(&"Test2").unwrap();
        assert_eq!(removed_value, 10);
        assert_eq!(x.counter, 2);
    }

    #[test]
    fn remove_last_element() {
        let mut x = HashLinkedList::new();
        x.add_first("Test1", 5);
        x.add_first("Test2", 10);
        x.add_first("Test3", 15);
        let removed_value = x.remove(&"Test1").unwrap();
        assert_eq!(removed_value, 5);
        assert_eq!(x.counter, 2);
    }
}

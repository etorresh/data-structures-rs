/**
 * Singly Linked List modified to work wih a Hash Map
 */

type Link<K: PartialEq, V> = Option<Box<Node<K, V>>>;
struct Node<K: PartialEq, V> {
    key: K,
    value: V,
    next: Link<K, V>,
}

pub struct HashLinkedList<K: PartialEq, V> {
    head: Link<K, V>,
}

impl<K: PartialEq, V> HashLinkedList<K, V> {
    pub fn new() -> HashLinkedList<K, V> {
        let head = None;
        HashLinkedList { head }
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
                    return removed_node.map(|node| node.value);
                }
                Some(node) => {
                    current_link = &mut node.next;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_list() {
        let list = HashLinkedList::<&str, &str>::new();
        assert_eq!(list.get(&"key1"), None);
    }

    #[test]
    fn insert_single_element() {
        let mut list = HashLinkedList::new();
        assert_eq!(list.insert("key1", "value1"), None);
        assert_eq!(list.get(&"key1"), Some(&"value1"));
    }

    #[test]
    fn insert_multiple_elements() {
        let mut list = HashLinkedList::new();
        list.insert("key1", "value1");
        list.insert("key2", "value2");
        list.insert("key3", "value3");
        assert_eq!(list.get(&"key1"), Some(&"value1"));
        assert_eq!(list.get(&"key2"), Some(&"value2"));
        assert_eq!(list.get(&"key3"), Some(&"value3"));
    }

    #[test]
    fn overwrite_existing_key() {
        let mut list = HashLinkedList::new();
        list.insert("key1", "value1");
        assert_eq!(list.insert("key1", "new_value1"), Some("value1"));
        assert_eq!(list.get(&"key1"), Some(&"new_value1"));
    }

    #[test]
    fn remove_key_not_present() {
        let mut list: HashLinkedList<&str, i32> = HashLinkedList::new();
        assert_eq!(list.remove(&"key1"), None);
    }

    #[test]
    fn remove_single_key_from_single_element_list() {
        let mut list = HashLinkedList::new();
        list.insert("key1", "value1");
        assert_eq!(list.remove(&"key1"), Some("value1"));
        assert_eq!(list.get(&"key1"), None);
    }

    #[test]
    fn remove_first_key_from_multi_element_list() {
        let mut list = HashLinkedList::new();
        list.insert("key1", "value1");
        list.insert("key2", "value2");
        list.insert("key3", "value3");
        assert_eq!(list.remove(&"key1"), Some("value1"));
        assert_eq!(list.get(&"key1"), None);
        assert_eq!(list.get(&"key2"), Some(&"value2"));
        assert_eq!(list.get(&"key3"), Some(&"value3"));
    }

    #[test]
    fn remove_middle_key_from_multi_element_list() {
        let mut list = HashLinkedList::new();
        list.insert("key1", "value1");
        list.insert("key2", "value2");
        list.insert("key3", "value3");
        assert_eq!(list.remove(&"key2"), Some("value2"));
        assert_eq!(list.get(&"key1"), Some(&"value1"));
        assert_eq!(list.get(&"key2"), None);
        assert_eq!(list.get(&"key3"), Some(&"value3"));
    }

    #[test]
    fn remove_last_key_from_multi_element_list() {
        let mut list = HashLinkedList::new();
        list.insert("key1", "value1");
        list.insert("key2", "value2");
        list.insert("key3", "value3");
        assert_eq!(list.remove(&"key3"), Some("value3"));
        assert_eq!(list.get(&"key1"), Some(&"value1"));
        assert_eq!(list.get(&"key2"), Some(&"value2"));
        assert_eq!(list.get(&"key3"), None);
    }

    #[test]
    fn remove_key_twice() {
        let mut list = HashLinkedList::new();
        list.insert("key1", "value1");
        assert_eq!(list.remove(&"key1"), Some("value1"));
        assert_eq!(list.remove(&"key1"), None);
    }

    #[test]
    fn complex_sequence_of_operations() {
        let mut list = HashLinkedList::new();
        list.insert("key1", "value1");
        list.insert("key2", "value2");
        assert_eq!(list.get(&"key1"), Some(&"value1"));
        assert_eq!(list.insert("key2", "new_value2"), Some("value2"));
        assert_eq!(list.get(&"key2"), Some(&"new_value2"));
        list.insert("key3", "value3");
        assert_eq!(list.remove(&"key1"), Some("value1"));
        assert_eq!(list.get(&"key1"), None);
        assert_eq!(list.remove(&"key1"), None);
        assert_eq!(list.remove(&"key2"), Some("new_value2"));
        assert_eq!(list.get(&"key3"), Some(&"value3"));
    }
}

/**
 * A singly linked list optimized for use in a hash map.
 *
 * This structure is designed to be used as buckets for our hashmap. As the
 * hashmap encounters collisions on keys, the colliding elements are placed
 * into one of these linked lists, hence the need for the key to be stored
 * alongside the value.
 */

type Link<K, V> = Option<Box<Node<K, V>>>;
struct Node<K: PartialEq, V> {
    key: K,
    value: V,
    next: Link<K, V>,
}

pub struct HashLinkedList<K: PartialEq, V> {
    head: Link<K, V>,
}

// Iterator to consume the linked list and produce elements.
pub struct IntoIter<K: PartialEq, V>(HashLinkedList<K, V>);

impl<K: PartialEq, V> HashLinkedList<K, V> {
    // Creates a new, empty linked list.
    pub fn new() -> HashLinkedList<K, V> {
        let head = None;
        HashLinkedList { head }
    }

    // Inserts a key-value pair into the linked list.
    // If the key already exists, the old value is returned.
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
            value,
            next: None,
        }));

        *current_link = new_node;
        None
    }

    // Fetches a reference to the value associated with the provided key.
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

    // Removes the node associated with the provided key and returns its value.
    // If no such node exists, returns None.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut current_link = &mut self.head;
        loop {
            match current_link {
                None => return None,
                Some(node) if &node.key == key => {
                    let mut removed_node = current_link.take();
                    /* We can dereference current_link and asign a new value in the line below because "node"  is dropped at this  point, since
                    we don't use it anywhere else, and node was the only borrow on *current_link.  This is not possible in while let Some(x) = y because
                    *current_link is borrowed through the scope of the whole loop. */
                    *current_link = removed_node.as_mut().and_then(|node| node.next.take());
                    return removed_node.map(|node| node.value);
                }
                Some(node) => {
                    current_link = &mut node.next;
                }
            }
        }
    }

    // Removes and returns the first key-value pair from the list. Only used to consume the list through IntoIter.
    fn remove_first(&mut self) -> Option<(K, V)> {
        self.head.take().map(|node| {
            self.head = node.next;
            (node.key, node.value)
        })
    }

    // Consumes the list and returns an iterator over the list.
    pub fn into_iter(self) -> IntoIter<K, V> {
        IntoIter(self)
    }
}

impl<K: PartialEq, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.remove_first()
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

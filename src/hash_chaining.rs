// a hash implementation that uses chaining to avoid collisions.
use crate::singly_linked_list::LinkedList;
use std::collections::hash_map::DefaultHasher;
use std::collections::hash_map::RandomState;

pub struct HashMap<K, V> {
    hash_array: Vec<LinkedList<(K, V)>>,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> HashMap<K, V> {
        let hash_array = Vec::new();
        HashMap { hash_array }
    }
    pub fn with_capacity(capacity: usize) -> HashMap<K, V> {
        let hash_array = (0..capacity).map(|_| LinkedList::new()).collect();
        HashMap { hash_array }
    }

    // Returns old value if key was previously present
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        None
    }

    fn get(&self, key: K) -> Option<V> {
        None
    }

    // Returns removed value
    fn remove(&mut self, key: K) -> Option<V> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        assert!(true);
    }
}

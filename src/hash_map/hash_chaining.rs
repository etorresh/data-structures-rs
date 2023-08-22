// a hash implementation that uses chaining to avoid collisions.
use crate::hash_map::hash_linked_list::HashLinkedList;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{self, Hash, Hasher},
};

const MAX_LOAD_FACTOR: f32 = 0.75;

pub struct HashMap<K: Hash, V> {
    hash_array: Vec<LinkedList<(K, V)>>,
    size: usize,
}

impl<K: Hash, V> HashMap<K, V> {
    pub fn new() -> HashMap<K, V> {
        let hash_array = Vec::new();
        HashMap {
            hash_array,
            size: 0,
        }
    }
    pub fn with_capacity(capacity: usize) -> HashMap<K, V> {
        let hash_array = (0..capacity).map(|_| LinkedList::new()).collect();
        HashMap {
            hash_array,
            size: 0,
        }
    }

    // Returns old value if key was previously present
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.hash_array.len() == 0 {
            *self = HashMap::with_capacity(2);
        }

        // Load factor
        if ((self.size as f32 + 1.0) / self.hash_array.len() as f32) >= MAX_LOAD_FACTOR {
            self.rehash();
        }

        let index = self.bucket_index(&key);

        let bucket = &mut self.hash_array[index];

        // Come back here after implementing iter_mut in LinkedList
        //      - if the bucket is empty add the key and value
        //      - if the bucket already has the key then replace it
        //      - if the bucket doesn't have the key then add it

        self.size += 1;
        None
    }

    pub fn get(&self, key: K) -> Option<V> {
        None
    }

    // Returns removed value
    pub fn remove(&mut self, key: K) -> Option<V> {
        None
    }

    fn bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_value = hasher.finish();
        let bucket_index = (hash_value as usize) % self.hash_array.len();
        bucket_index
    }

    fn rehash(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        assert!(true);
    }
}

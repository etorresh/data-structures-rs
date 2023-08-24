// a hash implementation that uses chaining to avoid collisions.
use crate::hash_map::hash_linked_list::HashLinkedList;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const MAX_LOAD_FACTOR: f32 = 0.75;

pub struct HashMap<K: PartialEq + Hash, V> {
    hash_array: Vec<HashLinkedList<K, V>>,
    size: usize,
}

impl<K: PartialEq + Hash, V> HashMap<K, V> {
    pub fn new() -> HashMap<K, V> {
        let hash_array = Vec::new();
        HashMap {
            hash_array,
            size: 0,
        }
    }
    pub fn with_capacity(capacity: usize) -> HashMap<K, V> {
        let hash_array = (0..capacity).map(|_| HashLinkedList::new()).collect();
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

        let old_value = bucket.insert(key, value);

        if old_value.is_none() {
            self.size += 1;
        }
        old_value
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if self.hash_array.len() == 0 {
            return None;
        }
        let index = self.bucket_index(&key);
        let bucket = &self.hash_array[index];
        bucket.get(key)
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
    fn hash_insert() {
        let mut hash_map: HashMap<String, i32> = HashMap::new();
        assert_eq!(hash_map.insert("key1".to_string(), 42), None); // New insertion
        assert_eq!(hash_map.insert("key1".to_string(), 100), Some(42)); // Updating existing key
    }

    #[test]
    fn hash_insert_large_data() {
        // Test for larger data to ensure there's no crash/panic
        let mut hash_map: HashMap<String, i32> = HashMap::new();

        for i in 0..10000 {
            assert_eq!(hash_map.insert(format!("key{}", i), i), None);
        }

        for i in 0..10000 {
            assert_eq!(hash_map.insert(format!("key{}", i), i + 10000), Some(i));
        }
    }

    #[test]
    fn hash_get() {
        let mut hash_map: HashMap<String, i32> = HashMap::new();

        // Initially, the hashmap is empty, so any key should return None.
        assert_eq!(hash_map.get(&"key1".to_string()), None);

        // Insert a key-value pair.
        hash_map.insert("key1".to_string(), 42);

        // Now, when we get the value using the same key, it should return the inserted value.
        assert_eq!(hash_map.get(&"key1".to_string()), Some(&42));

        // Querying with a different key should return None.
        assert_eq!(hash_map.get(&"key2".to_string()), None);
    }
}

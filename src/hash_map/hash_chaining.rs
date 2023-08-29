// A custom hash map implementation that resolves collisions using chaining.

use crate::hash_map::hash_linked_list::HashLinkedList;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const MAX_LOAD_FACTOR: f32 = 0.75;
pub struct HashMap<K: PartialEq + Hash, V> {
    // An array of linked lists (chained hash table).
    hash_array: Vec<HashLinkedList<K, V>>,
    // The current number of key-value pairs in the hash map.
    size: usize,
}

impl<K: PartialEq + Hash, V> HashMap<K, V> {
    // Creates an empty hash map.
    pub fn new() -> HashMap<K, V> {
        let hash_array = Vec::new();
        HashMap {
            hash_array,
            size: 0,
        }
    }

    // Creates a hash map with a specific capacity.
    pub fn with_capacity(capacity: usize) -> HashMap<K, V> {
        let hash_array = Self::initialize_hash_array(capacity);
        HashMap {
            hash_array,
            size: 0,
        }
    }

    fn initialize_hash_array(capacity: usize) -> Vec<HashLinkedList<K, V>> {
        (0..capacity).map(|_| HashLinkedList::new()).collect()
    }

    // Inserts a key-value pair into the hash map. If the key already exists,
    // the old value is returned.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.hash_array.len() == 0 {
            self.hash_array = Self::initialize_hash_array(2);
        }

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

    // Fetches the value associated with a given key, returning None if the key
    // doesn't exist.
    pub fn get(&self, key: &K) -> Option<&V> {
        if self.size == 0 {
            return None;
        }
        let index = self.bucket_index(&key);
        let bucket = &self.hash_array[index];
        bucket.get(key)
    }

    // Removes a key-value pair from the hash map based on a given key.
    // Returns the removed value if the key was found.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.size == 0 {
            return None;
        }
        let index = self.bucket_index(key);
        let bucket = &mut self.hash_array[index];
        let removed_value = bucket.remove(key);
        if removed_value.is_some() {
            self.size -= 1;
        }
        removed_value
    }

    // Computes the index of the bucket for a given key.
    fn bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_value = hasher.finish();
        let bucket_index = (hash_value as usize) % self.hash_array.len();
        bucket_index
    }

    // Resizes the hash array by doubling its capacity and rehashes all the key-value pairs.
    fn rehash(&mut self) {
        let new_capacity = self.hash_array.len() * 2;

        let new_hash_array: Vec<HashLinkedList<K, V>> = Self::initialize_hash_array(new_capacity);

        let old_hash_array = std::mem::replace(&mut self.hash_array, new_hash_array);
        self.size = 0;
        for bucket in old_hash_array {
            for (key, value) in bucket.into_iter() {
                self.insert(key, value);
            }
        }
    }
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

    // Test the removal of keys.
    #[test]
    fn hash_remove() {
        let mut hash_map: HashMap<String, i32> = HashMap::new();

        // Insert a key-value pair.
        hash_map.insert("key1".to_string(), 42);

        // Remove the key and check the returned value.
        assert_eq!(hash_map.remove(&"key1".to_string()), Some(42));

        // Ensure the key is no longer present.
        assert_eq!(hash_map.get(&"key1".to_string()), None);
    }

    // Test the behavior when the map is at its max load factor.
    #[test]
    fn hash_max_load_factor() {
        let mut hash_map: HashMap<String, i32> = HashMap::with_capacity(4);
        for i in 0..3 {
            hash_map.insert(format!("key{}", i), i);
        }

        // At this point, the load factor is 0.75. Inserting another element should trigger rehash.
        hash_map.insert("key_trigger".to_string(), 42);

        // Verify all inserted keys are still present after rehash.
        for i in 0..3 {
            assert_eq!(hash_map.get(&format!("key{}", i)), Some(&i));
        }
        assert_eq!(hash_map.get(&"key_trigger".to_string()), Some(&42));
    }
}

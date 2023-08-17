// a hash implementation that uses chaining to avoid collisions. Uses a hasher from the standard library. TO DO: Implement my own hasher
use crate::singly_linked_list::LinkedList;
use std::collections::hash_map::DefaultHasher;
use std::collections::hash_map::RandomState;
use std::ops::Index;

pub struct HashMap<T> {
    hash_array: Vec<LinkedList<T>>,
}

impl<T> HashMap<T> {
    fn insert() {}
    fn remove() {}
    fn get() {}
}

// impl<T> Index<String> for HashMap<T> {
//     type Output = T;
//     fn index(self, index: String) -> &Self::Output {
//         let x = self.hash_array[0].remove_first();
//         match &x {}
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        assert!(true);
    }
}

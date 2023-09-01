/// `Heap<T>`: A binary heap data structure implementation.
///
/// This heap is implemented as a max heap, which means that the maximum element can be efficiently
/// accessed and removed. A binary heap is usually implemented using an array or a vector where
/// each element has a defined position based on its insertion order. In the case of this
/// `Heap<T>`, a `Vec<T>` is used.
///
/// # Future Enhancements
///
/// - Custom comparators: While the current implementation is hard-coded as a max heap,
///   there are plans to allow custom comparators to support both max and min heap operations.
use std::cmp::Ordering;

pub struct Heap<T> {
    items: Vec<T>,
}

impl<T: Ord> Heap<T> {
    pub fn new() -> Self {
        Heap { items: Vec::new() }
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        let added_value_index = self.items.len() - 1;
        self.bubble_up(added_value_index);
    }

    fn bubble_up(&mut self, child_index: usize) {
        if let Some(parent_index) = Self::parent_index(child_index) {
            self.bubble_up_from(child_index, parent_index);
        }
    }

    fn bubble_up_from(&mut self, child_index: usize, parent_index: usize) {
        match self.items[parent_index].cmp(&self.items[child_index]) {
            Ordering::Equal | Ordering::Greater => return,
            Ordering::Less => {
                let (left, right) = self.items.split_at_mut(child_index);
                let parent_value = &mut left[parent_index];
                let added_value = &mut right[0];
                std::mem::swap(added_value, parent_value);
                self.bubble_up(parent_index);
            }
        }
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }
        let root = self.items.swap_remove(0);
        self.bubble_down(0);

        Some(root)
    }

    fn bubble_down(&mut self, parent_index: usize) {
        let len = self.items.len();
        let left_child_index = Self::left_child_index(parent_index);
        let right_child_index = Self::right_child_index(parent_index);

        let mut target = None;

        // In a complete binary tree, if a right child exists, a left child must also exist.
        // Thus, we won't have a scenario where only the right_child_index is valid.
        // Check if the right child's index is within bounds of the array
        if right_child_index < len {
            // If the right child's value is greater than the left child's value,
            // set the target index to the right child's index
            if self.items[right_child_index] > self.items[left_child_index] {
                target = Some(right_child_index);
            }
            // Otherwise, set the target indedx to the left child's index.
            else {
                target = Some(left_child_index);
            }
        }
        // If the right child doesn't exist (is out of bounds), but the left child
        // is within bounds, set the target index to the left child's index.
        else if left_child_index < len {
            target = Some(left_child_index);
        }

        if let Some(target_index) = target {
            if self.items[target_index] > self.items[parent_index] {
                self.items.swap(parent_index, target_index);
                self.bubble_down(target_index);
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut heap = Heap { items: vec };
        for i in (0..heap.items.len()).rev() {
            heap.bubble_down(i);
        }
        heap
    }

    fn parent_index(child_index: usize) -> Option<usize> {
        if child_index == 0 {
            return None;
        }
        Some((child_index - 1) / 2)
    }

    fn left_child_index(parent_index: usize) -> usize {
        (2 * parent_index) + 1
    }

    fn right_child_index(parent_index: usize) -> usize {
        (2 * parent_index) + 2
    }
}
#[cfg(test)]
mod tests {
    use super::Heap;

    #[test]
    fn basic_add() {
        let mut heap = Heap { items: Vec::new() };
        heap.add(5);
        assert_eq!(heap.items[0], 5);
    }

    #[test]
    fn multiple_adds_ascending() {
        let mut heap = Heap { items: Vec::new() };
        heap.add(1);
        heap.add(2);
        heap.add(3);
        assert_eq!(heap.items[0], 3);
    }

    #[test]
    fn multiple_adds_descending() {
        let mut heap = Heap { items: Vec::new() };
        heap.add(3);
        heap.add(2);
        heap.add(1);
        assert_eq!(heap.items[0], 3);
    }

    #[test]
    fn multiple_adds_random() {
        let mut heap = Heap { items: Vec::new() };
        heap.add(2);
        heap.add(3);
        heap.add(1);
        assert_eq!(heap.items[0], 3);
    }

    #[test]
    fn duplicates() {
        let mut heap = Heap { items: Vec::new() };
        heap.add(2);
        heap.add(2);
        assert_eq!(heap.items.contains(&2), true);
    }

    #[test]
    fn remove_from_empty_heap() {
        let mut heap: Heap<i32> = Heap { items: Vec::new() };
        assert_eq!(heap.remove(), None);
    }

    #[test]
    fn add_and_remove_single_item() {
        let mut heap = Heap { items: Vec::new() };
        heap.add(5);
        assert_eq!(heap.remove(), Some(5));
        assert_eq!(heap.remove(), None);
    }

    #[test]
    fn add_multiple_and_remove() {
        let mut heap = Heap { items: Vec::new() };
        heap.add(1);
        heap.add(2);
        heap.add(3);
        dbg!(&heap.items);
        assert_eq!(heap.remove(), Some(3));
        dbg!(&heap.items);
        assert_eq!(heap.remove(), Some(2));
        assert_eq!(heap.remove(), Some(1));
        assert_eq!(heap.remove(), None);
    }

    #[test]
    fn add_and_remove_mixed() {
        let mut heap = Heap { items: Vec::new() };
        heap.add(5);
        heap.add(1);
        heap.add(8);
        heap.add(3);
        assert_eq!(heap.remove(), Some(8));
        heap.add(7);
        assert_eq!(heap.remove(), Some(7));
        assert_eq!(heap.remove(), Some(5));
        heap.add(2);
        assert_eq!(heap.remove(), Some(3));
        assert_eq!(heap.remove(), Some(2));
        assert_eq!(heap.remove(), Some(1));
        assert_eq!(heap.remove(), None);
    }

    #[test]
    fn add_remove_with_duplicates() {
        let mut heap = Heap { items: Vec::new() };
        heap.add(5);
        heap.add(5);
        heap.add(2);
        heap.add(2);
        assert_eq!(heap.remove(), Some(5));
        assert_eq!(heap.remove(), Some(5));
        assert_eq!(heap.remove(), Some(2));
        assert_eq!(heap.remove(), Some(2));
        assert_eq!(heap.remove(), None);
    }
}

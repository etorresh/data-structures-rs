/// `Heap<T>`: A binary heap data structure implementation.
///
/// This heap's order is determined by the provided comparator function.
/// A binary heap is usually implemented using an array or a vector where
/// each element has a defined position based on its insertion order. In the case of this
/// `Heap<T>`, a `Vec<T>` is used.
///

pub struct Heap<T> {
    items: Vec<T>,
    /// A comparator function to determine the order of elements in the heap.
    /// It returns `true` if the first argument should come before the second argument in the heap order.
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T> {
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Heap {
            items: Vec::new(),
            comparator,
        }
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
        if (self.comparator)(&self.items[child_index], &self.items[parent_index]) {
            self.items.swap(parent_index, child_index);
            self.bubble_up(parent_index);
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

        // Determine which child should be compared with the parent.
        // If the right child exists and is more preferable than the left child,
        // the target becomes the right child, else it's the left child.
        let target_index = if right_child_index < len
            && (self.comparator)(
                &self.items[right_child_index],
                &self.items[left_child_index],
            ) {
            right_child_index
        } else {
            left_child_index
        };
        // If the target child index is within bounds and is more preferable than the parent,
        // we swap the parent and target child. After swapping, we recursively
        // bubble down.
        if target_index < len
            && (self.comparator)(&self.items[target_index], &self.items[parent_index])
        {
            self.items.swap(parent_index, target_index);
            self.bubble_down(target_index);
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }

    pub fn from_vec(vec: Vec<T>, comparator: fn(&T, &T) -> bool) -> Self {
        let mut heap = Heap {
            items: vec,
            comparator,
        };
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

    fn max_heap_comparator(a: &i32, b: &i32) -> bool {
        a > b
    }

    fn min_heap_comparator(a: &i32, b: &i32) -> bool {
        a < b
    }

    #[test]
    fn basic_add() {
        let mut heap = Heap::new(max_heap_comparator);
        heap.add(5);
        assert_eq!(heap.items[0], 5);
    }

    #[test]
    fn multiple_adds_ascending() {
        let mut heap = Heap::new(max_heap_comparator);
        heap.add(1);
        heap.add(2);
        heap.add(3);
        assert_eq!(heap.items[0], 3);
    }

    #[test]
    fn multiple_adds_descending() {
        let mut heap = Heap::new(max_heap_comparator);
        heap.add(3);
        heap.add(2);
        heap.add(1);
        assert_eq!(heap.items[0], 3);
    }

    #[test]
    fn multiple_adds_random() {
        let mut heap = Heap::new(max_heap_comparator);
        heap.add(2);
        heap.add(3);
        heap.add(1);
        assert_eq!(heap.items[0], 3);
    }

    #[test]
    fn duplicates() {
        let mut heap = Heap::new(max_heap_comparator);
        heap.add(2);
        heap.add(2);
        assert_eq!(heap.items.contains(&2), true);
    }

    #[test]
    fn remove_from_empty_heap() {
        let mut heap = Heap::new(max_heap_comparator);
        assert_eq!(heap.remove(), None);
    }

    #[test]
    fn add_and_remove_single_item() {
        let mut heap = Heap::new(max_heap_comparator);
        heap.add(5);
        assert_eq!(heap.remove(), Some(5));
        assert_eq!(heap.remove(), None);
    }

    #[test]
    fn add_multiple_and_remove() {
        let mut heap = Heap::new(max_heap_comparator);
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
        let mut heap = Heap::new(max_heap_comparator);
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
        let mut heap = Heap::new(max_heap_comparator);
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

    #[test]
    fn multiple_adds_ascending_min_heap() {
        let mut heap = Heap::new(min_heap_comparator);
        heap.add(1);
        heap.add(2);
        heap.add(3);
        assert_eq!(heap.items[0], 1);
    }

    #[test]
    fn multiple_adds_descending_min_heap() {
        let mut heap = Heap::new(min_heap_comparator);
        heap.add(3);
        heap.add(2);
        heap.add(1);
        assert_eq!(heap.items[0], 1);
    }

    #[test]
    fn peek_max_heap() {
        let mut heap = Heap::new(max_heap_comparator);
        heap.add(3);
        heap.add(2);
        heap.add(1);
        assert_eq!(heap.peek(), Some(&3));
    }

    #[test]
    fn peek_min_heap() {
        let mut heap = Heap::new(min_heap_comparator);
        heap.add(3);
        heap.add(2);
        heap.add(1);
        assert_eq!(heap.peek(), Some(&1));
    }
}

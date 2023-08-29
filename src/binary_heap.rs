use std::cmp::Ordering;

// Heap
// Takes a closures as a comparator to set the max or min heap (for now I'll skip this and hard code a max heap)
pub struct Heap<T> {
    items: Vec<T>,
}

impl<T: Ord> Heap<T> {
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
                let new_child_index = parent_index;
                self.bubble_up(new_child_index);
            }
        }
    }

    pub fn remove() -> Option<T> {
        None
    }

    fn parent_index(child_index: usize) -> Option<usize> {
        if child_index == 0 {
            return None;
        }
        Some((child_index - 1) / 2)
    }

    fn left_child_index(&self, parent_index: usize) -> usize {
        (2 * parent_index) + 1
    }

    fn right_child_index(&self, parent_index: usize) -> usize {
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
}

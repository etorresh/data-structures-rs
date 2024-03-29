/*
Given a singly-linked list, devise a time- and space-efficient algorithm to find the
mth-to-last element of the list. Implement your algorithm, taking care to handle relevant error conditions. Define mth to last such that when m = 0, the last element of
the list is returned.

my own notes: I'm assuming we don't have a counter since that would be too easy of a solution
*/
#[cfg(test)]
mod tests {
    use super::linked_list;

    #[test]
    fn last_elem() {
        let mut list = linked_list::LinkedList::new();
        for i in 0..100 {
            list.add_last(i);
        }
        assert_eq!(list.mth_to_last_element(0), Some(&99));
    }

    #[test]
    fn first_elem() {
        let mut list = linked_list::LinkedList::new();
        for i in 0..100 {
            list.add_last(i);
        }
        assert_eq!(list.mth_to_last_element(100), Some(&0));
    }

    #[test]
    fn middle_elem() {
        let mut list = linked_list::LinkedList::new();
        for i in 0..100 {
            list.add_last(i);
        }
        assert_eq!(list.mth_to_last_element(50), Some(&49));
    }

    #[test]
    fn empty_list() {
        let list: linked_list::LinkedList<i32> = linked_list::LinkedList::new();
        assert!(list.mth_to_last_element(100).is_none());
    }

    #[test]
    fn list_with_one_element() {
        let mut list = linked_list::LinkedList::new();
        list.add_last(1);
        assert_eq!(list.mth_to_last_element(0), Some(&1));
    }
}

// Modified version of safe_rust::linked_list_singly.rs
#[cfg(test)]
#[allow(dead_code)]
mod linked_list {
    pub struct IntoIter<T>(LinkedList<T>);

    type Link<T> = Option<Box<Node<T>>>;
    struct Node<T> {
        data: T,
        next: Link<T>,
    }

    pub struct LinkedList<T> {
        head: Link<T>,
    }

    impl<T> LinkedList<T> {
        pub fn new() -> LinkedList<T> {
            let head = None;
            LinkedList { head }
        }

        pub fn add_last(&mut self, data: T) {
            let new_tail_node = Some(Box::new(Node { data, next: None }));

            let mut current_link = &mut self.head;
            while let Some(node) = current_link {
                current_link = &mut node.next;
            }
            *current_link = new_tail_node;
        }

        pub fn mth_to_last_element(&self, m: usize) -> Option<&T> {
            let mut current_opt = self.head.as_ref();
            let mut window_opt = self.head.as_ref();

            // Advance current_opt 'm' nodes
            for _ in 0..m {
                current_opt = current_opt?.next.as_ref();
            }

            // Advance both pointers until `current_opt` reaches the end
            while current_opt.is_some() {
                current_opt = current_opt?.next.as_ref();
                // Covers case where we get the last element
                if current_opt.is_some() {
                    window_opt = window_opt.unwrap().next.as_ref();
                }
            }

            window_opt.map(|node| &node.data)
        }
    }
}

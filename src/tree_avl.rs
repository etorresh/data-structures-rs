/*
AVL Tree
- Difference in height between left and right must always be <= 1.
*/

struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    data: T,
}

pub struct TreeAVL<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> TreeAVL<T> {
    pub fn new() -> Self {
        TreeAVL { root: None }
    }

    pub fn insert(&mut self, data: T) {
        match &mut self.root {
            Some(node) => {
                Self::insert_recursive(node, &data);
            }
            None => {
                self.root = Some(Box::new(Node {
                    left: None,
                    right: None,
                    data,
                }));
            }
        }
    }

    fn insert_recursive(node: &mut Node<T>, data: &T) {
        todo!()
    }

    pub fn remove() {}

    pub fn search() {}
}

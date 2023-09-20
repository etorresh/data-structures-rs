/*
AVL Tree
- Difference in height between left and right must always be <= 1.
*/

use std::cmp::Ordering;
struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    data: T,
}

pub struct TreeAVL<T: Ord> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Ord> TreeAVL<T> {
    pub fn new() -> Self {
        TreeAVL {
            root: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, data: T) {
        let new_node = Node {
            left: None,
            right: None,
            data,
        };
        match &mut self.root {
            Some(node) => {
                if Self::insert_recursive(node, new_node) {
                    self.size += 1;
                }
            }
            None => {
                self.root = Some(Box::new(new_node));
                self.size += 1;
            }
        }
    }

    fn insert_recursive(parent: &mut Node<T>, new_node: Node<T>) -> bool {
        match &new_node.data.cmp(&parent.data) {
            Ordering::Less => {
                // add to left side
                match &mut parent.left {
                    Some(node) => {
                        return Self::insert_recursive(node, new_node);
                    }
                    None => {
                        parent.left = Some(Box::new(new_node));
                        return true;
                    }
                }
            }
            Ordering::Equal => return false,
            Ordering::Greater => {
                // add to right side
                match &mut parent.right {
                    Some(node) => {
                        return Self::insert_recursive(node, new_node);
                    }
                    None => {
                        parent.right = Some(Box::new(new_node));
                        return true;
                    }
                }
            }
        }
    }

    pub fn remove() {}

    pub fn search() {}
}

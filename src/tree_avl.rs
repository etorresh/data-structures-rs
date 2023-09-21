/*
AVL Tree
 Difference in height between left and right must always be <= 1.
 There are multiple approaches to handle equal values in AVL trees:
 1) Ignore equal values, 2) Allow duplicates, and 3) Store a counter for duplicates.
 For this implementation, I choose to ignore equal values to prioritize and simplify
 the focus on the tree's rebalancing logic.
*/

use std::cmp::{self, Ordering};
struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    height: usize,
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
            height: 0,
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
                        let inserted = Self::insert_recursive(node, new_node);
                        if inserted {
                            // rebalance
                            node.height = Self::height(node);
                        }
                        inserted
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
                // symmetric
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

    fn height(node: &Box<Node<T>>) -> usize {
        if node.left.is_none() && node.right.is_none() {
            return 0;
        }
        1 + cmp::max(
            node.left.as_ref().map_or(0, |node| node.height),
            node.right.as_ref().map_or(0, |node| node.height),
        )
    }

    fn rotate_ll() {}
    fn rotate_rr() {}
    fn rotate_lr() {}
    fn rotate_rl() {}

    pub fn remove() {}

    pub fn search() {}
}

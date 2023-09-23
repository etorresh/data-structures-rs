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

    fn insert_recursive(parent: &mut Box<Node<T>>, new_node: Node<T>) -> bool {
        let inserted = match &new_node.data.cmp(&parent.data) {
            Ordering::Less => {
                // add to left side
                match &mut parent.left {
                    Some(node) => Self::insert_recursive(node, new_node),
                    None => {
                        parent.left = Some(Box::new(new_node));
                        true
                    }
                }
            }
            Ordering::Equal => false,
            Ordering::Greater => {
                // add to right side
                match &mut parent.right {
                    Some(node) => Self::insert_recursive(node, new_node),
                    None => {
                        parent.right = Some(Box::new(new_node));
                        true
                    }
                }
            }
        };

        if inserted {
            parent.height = Self::height(parent);
            Self::check_balance(parent);
        }

        inserted
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

    fn balance_factor(node: &Box<Node<T>>) -> isize {
        let height_left = node.left.as_ref().map_or(0, Self::height);
        let height_right = node.right.as_ref().map_or(0, Self::height);
        let balance_factor: isize = (height_left - height_right).try_into().unwrap();
        balance_factor
    }

    fn check_balance(node: &mut Box<Node<T>>) {
        let balance_factor = Self::balance_factor(node);
        if balance_factor > 1 || balance_factor < -1 {
            Self::rebalance(node, balance_factor);
        }
    }

    fn rebalance(node: &mut Box<Node<T>>, balance_factor: isize) {}

    fn rotate_ll() {}
    fn rotate_rr() {}
    fn rotate_lr() {}
    fn rotate_rl() {}

    pub fn remove() {}

    pub fn search() {}
}

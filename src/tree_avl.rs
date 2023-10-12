/*
AVL Tree
 Difference in height between left and right must always be <= 1.
 There are multiple approaches to handle equal values in AVL trees:
 1) Ignore equal values, 2) Allow duplicates, and 3) Store a counter for duplicates.
 For this implementation, I choose to ignore equal values to prioritize and simplify
 the focus on the tree's rebalancing logic.

https://stackoverflow.com/questions/63452633/is-the-ll-rotation-a-single-left-rotation-or-a-single-right-rotation?rq=1
A tree with an LL imbalance needs a right rotation.
A tree with a RR imbalance needs a left rotation.
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
            node.left.as_ref().map_or(0, |child| child.height),
            node.right.as_ref().map_or(0, |child| child.height),
        )
    }

    fn balance_factor(node: &Box<Node<T>>) -> isize {
        let height_left: isize = node
            .left
            .as_ref()
            .map_or(-1, |child| child.height.try_into().unwrap());
        let height_right: isize = node
            .right
            .as_ref()
            .map_or(-1, |child| child.height.try_into().unwrap());

        let balance_factor: isize = height_left - height_right;
        balance_factor
    }

    fn check_balance(node: &mut Box<Node<T>>) {
        let balance_factor = Self::balance_factor(node);
        if balance_factor > 1 || balance_factor < -1 {
            Self::rebalance(node, balance_factor);
        }
    }

    /*
    Balance factor is positive:
        This indicates that the left subtree is taller than the right subtree.
        If the balance factor is 1, then the left subtree is just one level deeper than the right subtree.
        If the balance factor is greater than 1 then it's a sign the AVL property is violated, and the tree is too heavy on the left side.
    Balance factor is negative:
        This indicates that the right subtree is taller than the left subtree.
        If the balance factor is -1, then the right subtree is one level deeper than the left subtree.
        If the balance factor is less than -1, then it's a sign that the AVL property is violated, and the t ree is too heavy on the right side.
     */
    fn rebalance(node: &mut Box<Node<T>>, balance_factor: isize) {
        // Too heavy on the left side
        if balance_factor > 1 {
            let left_child_balance_factor = Self::balance_factor(node.left.as_ref().unwrap());
            if left_child_balance_factor >= 1 {
                Self::rotate_right(node);
            } else if left_child_balance_factor <= -1 {
                Self::rotate_left_then_right(node);
            } else {
                panic!("reached impossible case child balance factor is between -1 and 1 non-inclusive");
            }
        }
        // Too heavy on the right side
        else if balance_factor < -1 {
            let right_child_balance_factor = Self::balance_factor(node.right.as_ref().unwrap());
            if right_child_balance_factor <= -1 {
                Self::rotate_left(node);
            } else if right_child_balance_factor >= 1 {
                Self::rotate_right_then_left(node);
            } else {
                panic!("reached impossible case child balance factor is between -1 and 1 non-inclusive");
            }
        }
    }

    // Balance factor of the current node is < -1, and balance factor of the right child is <= -1.
    fn rotate_left(node: &mut Box<Node<T>>) {
        let mut right_child = node.right.take().unwrap();
        node.right = right_child.left.take();
        std::mem::swap(&mut right_child, node);

        // Update height and then attach as right child of node
        right_child.height = Self::height(&right_child);
        node.left = Some(right_child);

        // Update height
        node.height = Self::height(node);
    }

    // Balance factor of the current node is > 1, and balance factor of the right child is <= -1.
    fn rotate_right(node: &mut Box<Node<T>>) {
        let mut left_child = node.left.take().unwrap();
        node.left = left_child.right.take();
        std::mem::swap(&mut left_child, node);

        // Update height and then attach as right child of node
        left_child.height = Self::height(&left_child);
        node.right = Some(left_child);

        // Update height
        node.height = Self::height(node);
    }

    // Balance factor of the current node is < -1, and balance factor of the right child is >= 1.
    fn rotate_left_then_right(node: &mut Box<Node<T>>) {
        Self::rotate_left(node.left.as_mut().unwrap()); // change this line
        Self::rotate_right(node);
    }

    // Balance factor of the current node is > 1, and balance factor of the right child is >= 1.
    fn rotate_right_then_left(node: &mut Box<Node<T>>) {
        Self::rotate_right(node.right.as_mut().unwrap());
        Self::rotate_left(node);
    }

    pub fn remove() {}

    pub fn search() {}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_right_three_elements() {
        let mut x: TreeAVL<i32> = TreeAVL::new();
        x.insert(10);
        x.insert(7);
        x.insert(5);
        assert_eq!(x.root.as_ref().unwrap().data, 7);
        assert_eq!(x.root.as_ref().unwrap().left.as_ref().unwrap().data, 5);
        assert_eq!(x.root.as_ref().unwrap().right.as_ref().unwrap().data, 10);
    }

    #[test]
    fn rotate_left_three_elements() {
        let mut x: TreeAVL<i32> = TreeAVL::new();
        x.insert(5);
        x.insert(7);
        x.insert(10);
        assert_eq!(x.root.as_ref().unwrap().data, 7);
        assert_eq!(x.root.as_ref().unwrap().left.as_ref().unwrap().data, 5);
        assert_eq!(x.root.as_ref().unwrap().right.as_ref().unwrap().data, 10);
    }

    #[test]
    fn rotate_right_then_left_three_elements() {
        let mut x: TreeAVL<i32> = TreeAVL::new();
        x.insert(5);
        x.insert(10);
        x.insert(7);
        assert_eq!(x.root.as_ref().unwrap().data, 7);
        assert_eq!(x.root.as_ref().unwrap().left.as_ref().unwrap().data, 5);
        assert_eq!(x.root.as_ref().unwrap().right.as_ref().unwrap().data, 10);
    }

    #[test]
    fn rotate_left_then_right_three_elements() {
        let mut x: TreeAVL<i32> = TreeAVL::new();
        x.insert(10);
        x.insert(5);
        x.insert(7);
        assert_eq!(x.root.as_ref().unwrap().data, 7);
        assert_eq!(x.root.as_ref().unwrap().left.as_ref().unwrap().data, 5);
        assert_eq!(x.root.as_ref().unwrap().right.as_ref().unwrap().data, 10);
    }

    #[test]
    fn rotate_right_complete_case() {
        let mut x: TreeAVL<i32> = TreeAVL::new();
        x.insert(100);
        x.insert(40);
        x.insert(150);
        x.insert(200);
        x.insert(50);
        x.insert(26);
        x.insert(27);
        x.insert(25);
        x.insert(24);
        assert_eq!(x.root.as_ref().unwrap().data, 100);
        assert_eq!(x.root.as_ref().unwrap().right.as_ref().unwrap().data, 150);
        assert_eq!(
            x.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .data,
            200
        );
        assert_eq!(x.root.as_ref().unwrap().left.as_ref().unwrap().data, 26);
        assert_eq!(
            x.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .data,
            40
        );
        assert_eq!(
            x.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .data,
            25
        );
        assert_eq!(
            x.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .data,
            24
        );
        assert_eq!(
            x.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .data,
            27
        );
        assert_eq!(
            x.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .data,
            50
        );
    }

    #[test]
    fn rotate_left_complete_case() {
        let mut x: TreeAVL<i32> = TreeAVL::new();
        x.insert(100);
        x.insert(99);
        x.insert(150);
        x.insert(98);
        x.insert(125);
        x.insert(200);
        x.insert(175);
        x.insert(250);
        x.insert(300);
        assert_eq!(x.root.as_ref().unwrap().data, 100);
        assert_eq!(x.root.as_ref().unwrap().left.as_ref().unwrap().data, 99);
        assert_eq!(
            x.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .data,
            98
        );
        assert_eq!(x.root.as_ref().unwrap().right.as_ref().unwrap().data, 200);
        assert_eq!(
            x.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .data,
            150
        );
        assert_eq!(
            x.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .data,
            125
        );
        assert_eq!(
            x.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .data,
            175
        );
        assert_eq!(
            x.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .data,
            250
        );
        assert_eq!(
            x.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .data,
            300
        );
    }
}

use crate::LinkedList;

pub struct StackLinkedList<T> {
    list: LinkedList<T>,
}

impl<T> StackLinkedList<T> {
    pub fn new() -> Self {
        StackLinkedList {
            list: LinkedList::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.list.add_first(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.remove_first()
    }

    pub fn peek(&self) -> Option<&T> {
        self.list.peek()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.list.peek_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.list.size() == 0
    }

    pub fn size(&self) -> usize {
        self.list.size()
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::StackLinkedList;

    #[test]
    fn push_pop() {
        let mut stack = StackLinkedList::new(); // Assuming you have a 'new' method
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None); // Underflow
    }

    #[test]
    fn peek() {
        let mut stack = StackLinkedList::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
        stack.pop();
        assert_eq!(stack.peek(), Some(&1));
        stack.pop();
        assert_eq!(stack.peek(), None); // Stack is empty
    }

    #[test]
    fn size_and_is_empty() {
        let mut stack = StackLinkedList::new();
        assert_eq!(stack.is_empty(), true);
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.size(), 2);
        stack.pop();
        stack.pop();
        assert_eq!(stack.is_empty(), true);
    }

    #[test]
    fn clear() {
        let mut stack = StackLinkedList::new();
        stack.push(1);
        stack.push(2);
        stack.clear();
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek_mut() {
        let mut stack = StackLinkedList::new();

        // Stack is empty
        assert!(stack.peek_mut().is_none());

        stack.push(1);
        stack.push(2);

        // Modify the top of the stack
        *stack.peek_mut().unwrap() = 99;
        assert_eq!(stack.peek(), Some(&99));

        stack.pop();
        // Modify what's now the top of the stack
        *stack.peek_mut().unwrap() = 88;
        assert_eq!(stack.peek(), Some(&88));

        stack.pop();
        // Stack is now empty again
        assert!(stack.peek_mut().is_none());
    }
}

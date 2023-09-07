pub struct StackVec<T> {
    list: Vec<T>,
}

impl<T> StackVec<T> {
    pub fn new() -> Self {
        StackVec { list: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.list.push(item);
    }

    pub fn append(&mut self, items: &mut Vec<T>) {
        self.list.append(items)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        let len = self.list.len();
        if len == 0 {
            return None;
        }
        Some(&self.list[len - 1])
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        let len = self.list.len();
        if len == 0 {
            return None;
        }
        Some(&mut self.list[len - 1])
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn size(&self) -> usize {
        self.list.len()
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::StackVec;

    #[test]
    fn push_pop() {
        let mut stack = StackVec::new(); // Assuming you have a 'new' method
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None); // Underflow
    }

    #[test]
    fn peek() {
        let mut stack = StackVec::new();
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
        let mut stack = StackVec::new();
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
        let mut stack = StackVec::new();
        stack.push(1);
        stack.push(2);
        stack.clear();
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek_mut_empty() {
        let mut stack: StackVec<i32> = StackVec { list: Vec::new() };
        assert_eq!(stack.peek_mut(), None);
    }

    #[test]
    fn test_peek_mut_single_item() {
        let mut stack = StackVec { list: vec![42] };
        assert_eq!(*stack.peek_mut().unwrap(), 42);
    }

    #[test]
    fn test_peek_mut_multiple_items() {
        let mut stack = StackVec {
            list: vec![1, 2, 3, 4],
        };
        assert_eq!(*stack.peek_mut().unwrap(), 4);
    }

    #[test]
    fn test_peek_mut_modify_value() {
        let mut stack = StackVec {
            list: vec![1, 2, 3, 4],
        };
        *stack.peek_mut().unwrap() = 99;
        assert_eq!(stack.list[3], 99);
    }
}

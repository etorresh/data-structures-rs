use std::collections::VecDeque;

pub struct QueueVec<T> {
    list: VecDeque<T>,
}

impl<T> QueueVec<T> {
    pub fn new() -> Self {
        QueueVec {
            list: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.list.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.list.len() == 0
    }

    pub fn size(&self) -> usize {
        self.list.len()
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn peek(&self) -> Option<&T> {
        self.list.front()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.list.front_mut()
    }
}

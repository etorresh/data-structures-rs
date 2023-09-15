use std::cell::{Ref, RefMut};

use crate::LinkedListSinglyTail;

pub struct QueueLinkedList<T> {
    list: LinkedListSinglyTail<T>,
}

impl<T> QueueLinkedList<T> {
    pub fn new() -> Self {
        QueueLinkedList {
            list: LinkedListSinglyTail::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.list.add_last(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.list.remove_first()
    }

    pub fn peek(&self) -> Option<Ref<T>> {
        todo!()
    }

    pub fn peek_mut(&mut self) -> Option<RefMut<T>> {
        todo!()
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
use core::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug, Hash)]
pub struct Stack<E: Clone + Debug + Hash> {
    data: Vec<E>,
}

impl<E: Clone + Debug + Hash> Stack<E> {
    pub const fn new() -> Self {
        return Stack { data: Vec::new() };
    }

    pub fn push(&mut self, expr: E) {
        self.data.push(expr);
    }

    pub fn pop(&mut self) -> E {
        return self.data.pop().unwrap();
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn swap(&mut self, n: usize) {
        let stack_length = self.len();
        return self.data.swap(stack_length - 1, stack_length - 1 - n);
    }

    pub fn dup(&mut self, n: usize) {
        self.push(self.data[self.len() - n].clone());
    }

    pub fn peek(&self) -> E {
        return self.data[self.len() - 1].clone();
    }

    pub fn peek_at(&self, index: usize) -> E {
        return self.data[index].clone();
    }

    pub fn _get_data(&self) -> &Vec<E> {
        return &self.data;
    }
}

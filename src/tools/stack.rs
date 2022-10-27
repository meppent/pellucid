use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Stack<E: Debug + Clone + Debug + Hash + PartialEq + Eq> {
    data: Vec<E>,
}

impl<E: Debug + Eq + Clone + Debug + Hash + PartialEq> Stack<E> {
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

    pub fn peek(&self) -> &E {
        return &self.data[self.len() - 1];
    }

    pub fn peek_at(&self, depth: usize) -> &E {
        // depth = 0 => element on top of the stack
        return &self.data[self.len() - depth - 1];
    }

    pub fn _get_data(&self) -> &Vec<E> {
        return &self.data;
    }

    pub fn _down_push(&mut self, expr: E) {
        self.data.insert(0, expr);
    }

    pub fn iter(&self) -> impl Iterator<Item = &E> {
        return self.data.iter();
    }

    pub fn multi_pop(&mut self, n: usize) -> Vec<E> {
        let mut popped_values: Vec<E> = Vec::new();
        for _ in 0..n {
            popped_values.push(self.data.pop().unwrap());
        }
        return popped_values;
    }
}

// impl<E: Debug + Eq + Clone + Debug + Hash + PartialEq> std::ops::Deref for Stack<E> {
//     type Target = Vec<E>;
//     fn deref(&self) -> &Self::Target {
//         &self.data
//     }
// }

// impl<E: Debug + Eq + Clone + Debug + Hash + PartialEq> std::ops::DerefMut for Stack<E> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.data
//     }
// }

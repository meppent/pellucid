use core::fmt::Debug;
use std::hash::Hash;

use super::expressions::expression::Expression;

#[derive(Clone, Debug, Hash)]
pub struct Stack<E: Expression> {
    data: Vec<E>,
}

impl<Expr: Expression> Stack<Expr> {
    pub const fn new() -> Self {
        return Stack { data: Vec::new() };
    }

    pub fn push(&mut self, expr: Expr) {
        self.data.push(expr);
    }

    pub fn pop(&mut self) -> Expr {
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

    pub fn peek(&self) -> Expr {
        return self.data[self.len() - 1].clone();
    }

    pub fn _get_data(&self) -> &Vec<Expr> {
        return &self.data;
    }
}

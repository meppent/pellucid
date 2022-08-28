use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct Stack {
    data: Vec<Expression>,
}

impl Stack {
    pub const fn new() -> Self {
        return Stack { data: Vec::new() };
    }

    pub fn push(&mut self, expr: Expression) {
        self.data.push(expr);
    }

    pub fn pop(&mut self) -> Expression {
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

    pub fn peek(&self) -> Expression {
        return self.data[self.len() - 1].clone();
    }

    pub fn equals_on_bytes(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }
        for index in 0..self.len() {
            if let (Expression::VALUE(value0), Expression::VALUE(value1)) =
                (&self.data[index], &other.data[index])
            {
                if value0 != value1 {
                    return false;
                }
            }
        }

        return true;
    }
}

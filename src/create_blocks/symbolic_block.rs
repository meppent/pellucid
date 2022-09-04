use crate::bytecode_reader::{vopcode::Vopcode, opcode::Opcode};
use std::rc::Rc;

use crate::tools::stack::Stack;

use super::symbolic_expression::{SymbolicExpression, Effect};

#[derive(Debug)]
pub struct SymbolicBlock {
    stack: Stack<SymbolicExpression>,
    effects: Vec<Rc<Effect>>,
    impact: Option<SymbolicExpression>,
    n_args: usize,
}

impl Default for SymbolicBlock {
    fn default() -> Self { 
        SymbolicBlock::new()
     }
}

impl SymbolicBlock {
    pub fn new() -> Self {
        return SymbolicBlock {
            stack: Stack::new(),
            effects: Vec::new(),
            impact: None,
            n_args: 0,
        };
    }

    pub fn delta(&self) -> isize {
        return self.n_outputs() as isize - self.n_args as isize;
    }

    pub fn n_outputs(&self) -> usize {
        return self.stack.len();
    }

    fn fill_stack_with_place_holders(&mut self, required_input_count: usize) {
        while self.stack.len() < required_input_count {
            self.stack._down_push(
                SymbolicExpression::new_arg(self.n_args + 1, None)
            );
            self.n_args += 1;
        }
    }

    pub fn apply_vopcode(&mut self, vopcode: &Vopcode) {
        self.fill_stack_with_place_holders(vopcode.opcode.stack_input());
        
        match vopcode.opcode {
            Opcode::PUSH { item_size: _ } => {
                self.stack.push(
                    SymbolicExpression::new_bytes(vopcode.value.unwrap(), None)
                );
            },

            Opcode::DUP { depth } => self.stack.dup(depth),
            Opcode::SWAP { depth } => self.stack.swap(depth),
            Opcode::POP => {
                self.stack.pop();
            },

            opcode => {
                                
                let mut consumed_symbolic_expressions: Vec<SymbolicExpression> = Vec::new();

                for _ in 0..opcode.stack_input() {
                    consumed_symbolic_expressions.push(self.stack.pop());
                }

                let effect: Option<Rc<Effect>>;

                if opcode.has_effect() {
                    let effect_ref = Rc::new(Effect::COMPOSE(opcode, consumed_symbolic_expressions.clone()));
                    effect = Some(Rc::clone(&effect_ref));
                    self.effects.push(Rc::clone(&effect_ref));
                } else {
                    effect = None;
                };
                
                if opcode.is_exiting() || opcode.is_jump() {
                    self.impact = Some(
                        SymbolicExpression::new_compose(opcode, consumed_symbolic_expressions, effect)
                    );
                } else if opcode.stack_output() > 0 {
                    self.stack.push(
                        SymbolicExpression::new_compose(opcode, consumed_symbolic_expressions, effect)
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use primitive_types::U256;

    use crate::create_blocks::symbolic_expression::StackExpression;

    use crate::bytecode_reader::bytecode::Bytecode;
    use super::*;

    #[test]
    pub fn test_apply_add() {
        let bytecode = Bytecode::from("01");
        let mut block = SymbolicBlock::new();
        block.apply_vopcode(&bytecode.get_vopcode_at(0));
        assert_eq!(block.n_outputs(), 1);
        assert_eq!(block.delta(), -1);
        assert_eq!(block.n_args, 2);
        assert_eq!(block.effects, []);
        assert_eq!(block.impact, None);
        let symbolic_expression = block.stack.peek();
        assert_eq!(symbolic_expression.origin_effect, None);
        match &symbolic_expression.stack_expression {
            StackExpression::COMPOSE(opcode, consumed_symbolic_expressions) => {
                assert_eq!(*opcode, Opcode::ADD);
                assert_eq!(consumed_symbolic_expressions.len(), 2);
                assert_eq!(consumed_symbolic_expressions[0], SymbolicExpression::new_arg(1, None));
                assert_eq!(consumed_symbolic_expressions[1], SymbolicExpression::new_arg(2, None));
            },

            _ => panic!("Unexpected stack expression"),
        }
    }

    #[test]
    pub fn test_apply_swap4() {
        let bytecode = Bytecode::from("93");
        let mut block = SymbolicBlock::new();
        block.apply_vopcode(&bytecode.get_vopcode_at(0));
        assert_eq!(block.n_outputs(), 5);
        assert_eq!(block.delta(), 0);
        assert_eq!(block.n_args, 5);
        assert_eq!(block.effects, []);
        assert_eq!(block.impact, None);
        for i in 0..5 {
            let last_expr = block.stack.pop();
            if i == 0 || i == 4 {
                assert_eq!(last_expr, SymbolicExpression::new_arg(5 - i, None));
            }
            else {
                assert_eq!(last_expr, SymbolicExpression::new_arg(i + 1, None));
            }
            
        }  
    }

    #[test]
    pub fn test_apply_dup4() {
        let bytecode = Bytecode::from("83");
        let mut block = SymbolicBlock::new();
        block.apply_vopcode(&bytecode.get_vopcode_at(0));
        assert_eq!(block.n_outputs(), 5);
        assert_eq!(block.delta(), 1);
        assert_eq!(block.n_args, 4);
        assert_eq!(block.effects, []);
        assert_eq!(block.impact, None);

        for i in 0..5 {
            let last_expr = block.stack.pop();
            if i == 0 {
                assert_eq!(last_expr, SymbolicExpression::new_arg(4, None));
            }
            else {
                assert_eq!(last_expr, SymbolicExpression::new_arg(i, None));
            }
            
        } 
    }

    #[test]
    pub fn test_apply_call_reference() {
        let bytecode = Bytecode::from("f1");
        let mut block = SymbolicBlock::new();
        block.apply_vopcode(&bytecode.get_vopcode_at(0));
        assert_eq!(block.n_outputs(), 1);
        assert_eq!(block.delta(), -6);
        assert_eq!(block.n_args, 7);
        assert_eq!(block.impact, None);
        let symbolic_expr = block.stack.peek();
        assert_eq!(*symbolic_expr.origin_effect.as_ref().unwrap(), block.effects[0]);
    
    }

    #[test]
    pub fn test_apply_revert() {
        let bytecode = Bytecode::from("fd");
        let mut block = SymbolicBlock::new();
        block.apply_vopcode(&bytecode.get_vopcode_at(0));
        assert_eq!(block.n_outputs(), 0);
        assert_eq!(block.delta(), -2);
        assert_eq!(block.n_args, 2);
        assert_ne!(block.impact, None);
        dbg!(block);
    
    }

    #[test]
    pub fn test_fill_stack_with_place_holders1() {
        let mut symbolic_block = SymbolicBlock::new();
        symbolic_block.fill_stack_with_place_holders(3);
        assert_eq!(symbolic_block.n_args, 3);
        assert_eq!(symbolic_block.effects.len(), 0);
        assert_eq!(symbolic_block.stack.len(), 3);
        for i in 1..4 {
            assert_eq!(
                symbolic_block.stack.pop(),
                SymbolicExpression::new_arg(i, None)
            );
        }
    }

    #[test]
    pub fn test_fill_stack_with_place_holders2() {
        let mut symbolic_block = SymbolicBlock::new();
        symbolic_block.stack.push(SymbolicExpression::new_bytes(U256::from(5), None));
        symbolic_block.fill_stack_with_place_holders(3);
        assert_eq!(symbolic_block.n_args, 2);
        assert_eq!(symbolic_block.effects.len(), 0);
        assert_eq!(symbolic_block.stack.len(), 3);
        assert_eq!(
            symbolic_block.stack.pop(),
            SymbolicExpression::new_bytes(U256::from(5), None)
        );
        for i in 1..3 {
            assert_eq!(
                symbolic_block.stack.pop(),
                SymbolicExpression::new_arg(i, None)
            );
        }
    }

}

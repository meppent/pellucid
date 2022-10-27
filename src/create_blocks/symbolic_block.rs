use serde::{Deserialize, Serialize};

use crate::bytecode_reader::{opcode::Opcode, vopcode::Vopcode};
use std::rc::Rc;

use crate::tools::stack::Stack;

use super::symbolic_expression::{Effect, SymbolicExpression};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SymbolicBlock {
    pub symbolic_expressions: Stack<SymbolicExpression>,
    pub effects: Vec<Rc<Effect>>,
    pub n_args: usize,
}

impl SymbolicBlock {
    pub fn from(code: &[Vopcode]) -> Self {
        let mut symbolic_block = SymbolicBlock {
            symbolic_expressions: Stack::new(),
            effects: Vec::new(),
            n_args: 0,
        };

        for vopcode in code {
            symbolic_block.apply_vopcode(vopcode);
        }

        return symbolic_block;
    }

    pub fn delta(&self) -> isize {
        return self.n_outputs() as isize - self.n_args as isize;
    }

    pub fn final_effect(&self) -> Option<Rc<Effect>> {
        let length = self.effects.len();

        if length > 0 {
            if self.effects[length - 1].opcode.is_jump() {
                return Some(Rc::clone(&self.effects[length - 1]));
            } else if self.effects[length - 1].opcode.is_exiting() {
                return Some(Rc::clone(&self.effects[length - 1]));
            }
        }
        return None;
    }

    pub fn n_outputs(&self) -> usize {
        return self.symbolic_expressions.len();
    }

    fn fill_stack_with_place_holders(&mut self, required_input_count: usize) {
        while self.symbolic_expressions.len() < required_input_count {
            self.symbolic_expressions
                ._down_push(SymbolicExpression::new_arg(self.n_args + 1, None));
            self.n_args += 1;
        }
    }

    pub fn apply_vopcode(&mut self, vopcode: &Vopcode) {
        self.fill_stack_with_place_holders(vopcode.opcode.stack_input());

        match vopcode.opcode {
            Opcode::DUP { depth } => self.symbolic_expressions.dup(depth),
            Opcode::SWAP { depth } => self.symbolic_expressions.swap(depth),
            Opcode::POP => {
                self.symbolic_expressions.pop();
            }
            Opcode::PUSH { item_size: _ } => {
                self.symbolic_expressions
                    .push(SymbolicExpression::new_bytes(vopcode.value.unwrap(), None));
            }

            opcode => {
                let consumed_symbolic_expressions: Vec<SymbolicExpression> =
                    self.symbolic_expressions.multi_pop(opcode.stack_input());

                let effect: Option<Rc<Effect>>;

                if opcode.has_effect() {
                    let effect_ref: Rc<Effect> = Rc::new(Effect {
                        opcode: opcode,
                        symbolic_expressions: consumed_symbolic_expressions.clone(),
                    });
                    effect = Some(Rc::clone(&effect_ref));
                    self.effects.push(Rc::clone(&effect_ref));
                } else {
                    effect = None
                };

                if opcode.stack_output() > 0 {
                    self.symbolic_expressions
                        .push(SymbolicExpression::new_compose(
                            opcode,
                            consumed_symbolic_expressions,
                            effect,
                        ))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::bytecode_reader::bytecode::Bytecode;

    #[test]
    pub fn test_apply_first_block() {
        let bytecode = Bytecode::from("608060405234801561001057").unwrap();
        let mut block = SymbolicBlock::from(&[]);
        block.apply_vopcode(&bytecode.get_vopcode_at(0));
        block.apply_vopcode(&bytecode.get_vopcode_at(2));
        block.apply_vopcode(&bytecode.get_vopcode_at(4));
        block.apply_vopcode(&bytecode.get_vopcode_at(5));
        block.apply_vopcode(&bytecode.get_vopcode_at(6));
        block.apply_vopcode(&bytecode.get_vopcode_at(7));
        block.apply_vopcode(&bytecode.get_vopcode_at(8));
        block.apply_vopcode(&bytecode.get_vopcode_at(11));
    }
}

// #[test]
// pub fn test_apply_swap4() {
//     let bytecode = Bytecode::from("93");
//     let mut block = SymbolicBlock::new();
//     block.apply_vopcode(&bytecode.get_vopcode_at(0));
//     assert_eq!(block.n_outputs(), 5);
//     assert_eq!(block.delta(), 0);
//     assert_eq!(block.n_args, 5);
//     assert_eq!(block.effects, []);
//     assert_eq!(block.final_state(), None);
//     for i in 0..5 {
//         let last_expr = block.stack.pop();
//         if i == 0 || i == 4 {
//             assert_eq!(last_expr, SymbolicExpression::new_arg(5 - i, None));
//         }
//         else {
//             assert_eq!(last_expr, SymbolicExpression::new_arg(i + 1, None));
//         }

//     }
// }

// #[test]
// pub fn test_apply_dup4() {
//     let bytecode = Bytecode::from("83");
//     let mut block = SymbolicBlock::new();
//     block.apply_vopcode(&bytecode.get_vopcode_at(0));
//     assert_eq!(block.n_outputs(), 5);
//     assert_eq!(block.delta(), 1);
//     assert_eq!(block.n_args, 4);
//     assert_eq!(block.effects, []);
//     assert_eq!(block.final_state(), None);

//     for i in 0..5 {
//         let last_expr = block.stack.pop();
//         if i == 0 {
//             assert_eq!(last_expr, SymbolicExpression::new_arg(4, None));
//         }
//         else {
//             assert_eq!(last_expr, SymbolicExpression::new_arg(i, None));
//         }

//     }
// }

// #[test]
// pub fn test_apply_call_reference() {
//     let bytecode = Bytecode::from("f1");
//     let mut block = SymbolicBlock::new();
//     block.apply_vopcode(&bytecode.get_vopcode_at(0));
//     assert_eq!(block.n_outputs(), 1);
//     assert_eq!(block.delta(), -6);
//     assert_eq!(block.n_args, 7);
//     assert_eq!(block.final_state(), None);
//     let symbolic_expr = block.stack.peek();
//     assert_eq!(*symbolic_expr.origin_effect.as_ref().unwrap(), block.effects[0]);
// }

// #[test]
// pub fn test_apply_revert() {
//     let bytecode = Bytecode::from("fd");
//     let mut block = SymbolicBlock::new();
//     block.apply_vopcode(&bytecode.get_vopcode_at(0));
//     assert_eq!(block.n_outputs(), 0);
//     assert_eq!(block.delta(), -2);
//     assert_eq!(block.n_args, 2);
//     assert_ne!(block.final_state(), None);
//     match (*block.final_state().unwrap()).borrow() {
//         Effect::COMPOSE(opcode, _consumed_symbolic_expressions) => {
//             assert_eq!(*opcode, Opcode::REVERT);
//         },
//     }
//     assert_eq!(block.final_state().unwrap(), block.effects[0]);
// }

// #[test]
// pub fn test_apply_mstore() {
//     let bytecode = Bytecode::from("52");
//     let mut block = SymbolicBlock::new();
//     block.apply_vopcode(&bytecode.get_vopcode_at(0));
//     assert_eq!(block.n_outputs(), 0);
//     assert_eq!(block.delta(), -2);
//     assert_eq!(block.n_args, 2);
//     assert_eq!(block.final_state(), None);
//     assert_eq!(block.stack.len(), 0);
//     assert_eq!(block.effects.len(), 1);
//     let effect = (*block.effects[0]).borrow();
//     match effect {
//         Effect::COMPOSE(opcode, consumed_symbolic_expressions) => {
//             assert_eq!(*opcode, Opcode::MSTORE);
//             assert_eq!(*consumed_symbolic_expressions, vec![SymbolicExpression::new_arg(1, None), SymbolicExpression::new_arg(2, None)]);
//         },
//     }
// }

//     #[test]
//     pub fn test_fill_stack_with_place_holders1() {
//         let mut symbolic_block = SymbolicBlock::new();
//         symbolic_block.fill_stack_with_place_holders(3);
//         assert_eq!(symbolic_block.n_args, 3);
//         assert_eq!(symbolic_block.effects.len(), 0);
//         assert_eq!(symbolic_block.stack.len(), 3);
//         for i in 1..4 {
//             assert_eq!(
//                 symbolic_block.stack.pop(),
//                 SymbolicExpression::new_arg(i, None)
//             );
//         }
//     }

//     #[test]
//     pub fn test_fill_stack_with_place_holders2() {
//         let mut symbolic_block = SymbolicBlock::new();
//         symbolic_block.stack.push(SymbolicExpression::new_bytes(U256::from(5), None));
//         symbolic_block.fill_stack_with_place_holders(3);
//         assert_eq!(symbolic_block.n_args, 2);
//         assert_eq!(symbolic_block.effects.len(), 0);
//         assert_eq!(symbolic_block.stack.len(), 3);
//         assert_eq!(
//             symbolic_block.stack.pop(),
//             SymbolicExpression::new_bytes(U256::from(5), None)
//         );
//         for i in 1..3 {
//             assert_eq!(
//                 symbolic_block.stack.pop(),
//                 SymbolicExpression::new_arg(i, None)
//             );
//         }
//     }

//     #[test]
//     pub fn test_apply_multiple_no_effect() {
//         let bytecode = Bytecode::from("01010150");
//         let mut block = SymbolicBlock::new();
//         for vopcode in bytecode.get_vopcodes() {
//             block.apply_vopcode(&vopcode);
//         }
//         assert_eq!(block.n_outputs(), 0);
//         assert_eq!(block.delta(), -4);
//         assert_eq!(block.n_args, 4);
//         assert_eq!(block.final_state(), None);
//         assert_eq!(block.effects.len(), 0);
//         assert_eq!(block.stack.len(), 0);
//     }

// }

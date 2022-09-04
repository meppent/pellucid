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
    n_outputs: usize,
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
            n_outputs: 0,
        };
    }

    pub fn delta(&self) -> isize {
        return self.n_outputs as isize - self.n_args as isize;
    }

    pub fn add_place_holder_on_stack(&mut self){
        self.stack._down_push(SymbolicExpression::new_arg(self.n_args + 1, None));
        self.n_args += 1;
    }

    pub fn add_vopcode(&mut self, vopcode: &Vopcode) {
        match vopcode.opcode {
            Opcode::PUSH { item_size: _ } => {
                self.stack.push(
                    SymbolicExpression::new_bytes(vopcode.value.unwrap(), None)
                );
            }

            Opcode::DUP { depth } => {
                while self.stack.len() < depth {
                    self.add_place_holder_on_stack();
                }
                self.stack.dup(depth);
            },

            Opcode::SWAP { depth } => {
                while self.stack.len() < depth + 1 {
                    self.add_place_holder_on_stack();
                }
                self.stack.swap(depth)
            },

            Opcode::POP => {
                if self.stack.len() == 0 {
                    self.add_place_holder_on_stack();
                } 
                self.stack.pop();
            },

            opcode => {
                
                let opcode_n_args = opcode.stack_input();
                let initial_len = self.stack.len();
                let local_delta = if opcode_n_args > initial_len {
                    opcode_n_args - initial_len
                } else {
                    0
                };

                let mut symbolic_expressions: Vec<SymbolicExpression> = Vec::new();

                // IDEA ? use add_place_holder_on_stack here, to avoid complicated maths (self.n_args + i - initial_len + 1, and self.n_args += local_delta;)
                for i in 0..opcode_n_args {
                    if i < initial_len {
                        symbolic_expressions.push(self.stack.pop());
                    } else {
                        symbolic_expressions.push(SymbolicExpression::new_arg(self.n_args + i - initial_len + 1, None))
                    }
                }
                self.n_args += local_delta;

                let effect: Option<Rc<Effect>>;

                if opcode.has_effect(){
                    let effect_ref = Rc::new(Effect::COMPOSE(opcode, symbolic_expressions.clone()));
                    effect = Some(Rc::clone(&effect_ref));
                    self.effects.push(Rc::clone(&effect_ref));
                } else {
                    effect = None;
                };

                if opcode.stack_output() > 0 {
                    self.stack.push(SymbolicExpression::new_compose(opcode, symbolic_expressions.clone(), effect))
                }
                //not 100% sure it's an else
                else if opcode.is_exiting() || opcode.is_jump() {
                    self.impact = Some(SymbolicExpression::new_compose(opcode, symbolic_expressions, effect));
                }
            }
        }
    }
}

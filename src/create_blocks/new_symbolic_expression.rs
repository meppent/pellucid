use std::{rc::Rc, fs};

use primitive_types::U256;

use crate::{
    bytecode_reader::{
        opcode::{Opcode, self},
        vopcode::Vopcode, bytecode::Bytecode,
    },
    tools::stack::Stack,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SymbolicExpression {
    stack_expression: StackExpression,
    effect: Option<Rc<Effect>>,
}

impl SymbolicExpression {
    pub fn new(stack_expression: StackExpression, effect: Option<Rc<Effect>>) -> Self {
        return SymbolicExpression {
            stack_expression,
            effect,
        };
    }

    pub fn new_bytes(value: U256, effect: Option<Rc<Effect>>) -> Self {
        return SymbolicExpression::new(StackExpression::BYTES(value), effect);
    }

    pub fn new_compose(opcode: Opcode, args: Vec<SymbolicExpression>, effect: Option<Rc<Effect>>) -> Self {
        return SymbolicExpression::new(StackExpression::COMPOSE(opcode, args), effect);
    }

    pub fn new_arg(index: usize, effect: Option<Rc<Effect>>) -> Self {
        return SymbolicExpression::new(StackExpression::ARG(index), effect);
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum StackExpression {
    BYTES(U256),
    COMPOSE(Opcode, Vec<SymbolicExpression>),
    ARG(usize),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Effect {
    COMPOSE(Opcode, Vec<SymbolicExpression>),
}
#[derive(Debug)]
pub struct SymbolicBlock {
    symbolic_expressions: Stack<SymbolicExpression>,
    effects: Vec<Rc<Effect>>,
    n_args: usize,
    n_outputs: usize,
}

impl SymbolicBlock {
    pub fn from(code: &[Vopcode]) -> Self {
        let mut symbolic_block: SymbolicBlock = SymbolicBlock {
            symbolic_expressions: Stack::new(),
            effects: Vec::new(),
            n_args: 0,
            n_outputs: 0,
        };

        for vopcode in code {
            symbolic_block.add_vopcode(vopcode);

            // TODO remove when tests are set up
            if vopcode.opcode.is_exiting() || vopcode.opcode.is_jump() {
                assert!(vopcode.pc == code[code.len()-1].pc);
            }

        }
        symbolic_block.n_outputs = symbolic_block.len() + symbolic_block.n_args;

        

        return symbolic_block;
    }

    pub fn delta(&self) -> isize {
        return self.n_outputs as isize - self.n_args as isize;
    }

    pub fn add_place_holder_on_stack(&mut self){
        self._down_push(SymbolicExpression::new_arg(self.n_args + 1, None));
        self.n_args += 1;
    }

    pub fn add_vopcode(&mut self, vopcode: &Vopcode) {
        match vopcode.opcode {
            Opcode::PUSH { item_size: _ } => {
                self.symbolic_expressions
                    .push(SymbolicExpression::new_bytes(vopcode.value.unwrap(), None));
            }

            Opcode::DUP { depth } => {
                if self.len() == 0 {
                    self.add_place_holder_on_stack();
                } 
                self.dup(depth)
            },
            
            Opcode::SWAP { depth } => {
                while self.len() < depth + 1 {
                    self.add_place_holder_on_stack();
                }
                self.swap(depth)
            },

            Opcode::POP => {
                if self.len() == 0 {
                    self.add_place_holder_on_stack();
                } 
                self.pop();
            },

            opcode => {
                
                let opcode_n_args = opcode.stack_input();
                let initial_len = self.len();
                let local_delta = if opcode_n_args > self.len() {opcode_n_args - initial_len} else { 0 };
                let mut symbolic_expressions: Vec<SymbolicExpression> = Vec::new();

                // IDEA ? use add_place_holder_on_stack here, to avoid complicated maths (self.n_args + i - initial_len + 1, and self.n_args += local_delta;)
                for i in 0..opcode_n_args {
                    if i < initial_len {
                        symbolic_expressions.push(self.pop());
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
                }else{
                    effect = None;
                };

                if opcode.stack_output() > 0 {
                    self.push(SymbolicExpression::new_compose(opcode, symbolic_expressions, effect))
                }
                }
            }
            
    }
    

    pub fn len(&self) -> usize {
        return self.symbolic_expressions.len();
    }

    pub fn pop(&mut self) -> SymbolicExpression {
        return self.symbolic_expressions.pop();
    }

    pub fn push(&mut self, symbolic_expression: SymbolicExpression) {
        self.symbolic_expressions.push(symbolic_expression);
    }

    pub fn _down_push(&mut self, symbolic_expression: SymbolicExpression) {
        // add an element at the beginning of the stack of symbolic_expressions
        self.symbolic_expressions._down_push(symbolic_expression);
    }

    pub fn peek(&self) -> &SymbolicExpression {
        return self.symbolic_expressions.peek();
    }

    pub fn swap(&mut self, depth: usize) {
        self.symbolic_expressions.swap(depth);
    }

    pub fn dup(&mut self, depth: usize) {
        self.symbolic_expressions.dup(depth);
        let mut to_change = self.symbolic_expressions.pop();
        to_change.effect = None;
        self.symbolic_expressions.push(to_change);
    }
}


#[test]
pub fn test(){
    let bytecode_string: String =
            fs::read_to_string("./assets/contracts/simple_contract/bytecode.txt")
                .expect("Unable to read file.");
    let bytecode: Bytecode = Bytecode::from(&bytecode_string);
    let vopcodes = bytecode.slice_code(16, 25);
    //dbg!(vopcodes);
    let block = SymbolicBlock::from(vopcodes);
    dbg!(block);
    
}


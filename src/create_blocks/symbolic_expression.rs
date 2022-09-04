use std::{rc::Rc, fs};

use primitive_types::U256;

use crate::bytecode_reader::opcode::Opcode;


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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SymbolicExpression {
    pub stack_expression: StackExpression,
    pub effect: Option<Rc<Effect>>,
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




// #[test]
// pub fn test(){
//     let bytecode_string: String =
//             fs::read_to_string("./assets/contracts/simple_contract/bytecode.txt")
//                 .expect("Unable to read file.");
//     let bytecode: Bytecode = Bytecode::from(&bytecode_string);
//     let vopcodes = bytecode.slice_code(16, 25);
//     //dbg!(vopcodes);
//     let block = SymbolicBlock::from(vopcodes);
//     dbg!(block);
    
// }


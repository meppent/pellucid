use primitive_types::U256;
use std::ops::{Deref, DerefMut};

use crate::{
    bytecode_reader::{opcode::Opcode, vopcode::Vopcode},
    execution_flow::execution_flow::FunctionLabel,
    tools::stack::Stack,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Variable {
    pub alias: usize,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Value {
    Calculation {
        opcode: Opcode,
        args: Vec<Value>,
    },
    Existing(Variable),
    Bytes(U256),
    FunctionReturnedValue {
        label: FunctionLabel,
        arguments: Vec<Value>,
        return_index: usize,
    },
}

impl Value {
    pub fn from_vars(vars: &Vec<Variable>) -> Vec<Value> {
        return vars
            .iter()
            .map(|var: &Variable| Value::Existing(*var))
            .collect();
    }

    pub fn is_bytes(&self) -> bool {
        if let Value::Bytes(_) = self {
            return true;
        }
        return false;
    }

    pub fn size(&self) -> usize {
        match self {
            Value::Calculation { opcode: _, args } => {
                1 + args.iter().map(|arg: &Value| arg.size()).sum::<usize>()
            }
            Value::Existing(_) => 1,
            Value::Bytes(_) => 1,
            Value::FunctionReturnedValue {
                label: _,
                arguments,
                return_index: _,
            } => {
                1 + arguments
                    .iter()
                    .map(|arg: &Value| arg.size())
                    .sum::<usize>()
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Line {
    Assignement {
        // 'receiving_var = assigned;'
        receiving_var: Option<Variable>, // None in case the value is dropped or when `assigned_value` is `Calculation` with an opcode that does not deposit a value on the stack
        assigned_value: Value,
    },
    If {
        condition: Value,
    },
    Empty,
}

impl Line {
    pub fn is_if(&self) -> bool {
        match self {
            Line::If { condition: _ } => true,
            _ => false,
        }
    }
}

pub static mut FREE_VAR_INDEX: usize = 0;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct VariablesStack {
    pub stack: Stack<Variable>,
}

impl VariablesStack {
    pub fn new() -> Self {
        let mut stack: Stack<Variable> = Stack::new();
        // usefull for debuging, will be removed when bugs are fixed
        for error_index in 0..100 {
            stack.push(Variable {
                alias: 6666000 + error_index, // 6666xxx -> error variables
            })
        }
        return VariablesStack { stack };
    }

    pub fn create_single_variable(&mut self) -> Variable {
        unsafe {
            FREE_VAR_INDEX += 1;

            return Variable {
                alias: FREE_VAR_INDEX - 1,
            };
        }
    }

    pub fn create_and_push_single_variable(&mut self) -> Variable {
        let var: Variable = self.create_single_variable();
        self.stack.push(var);
        return var;
    }

    pub fn create_and_push_vars(&mut self, n_variables: usize) -> Vec<Variable> {
        let mut variables: Vec<Variable> = Vec::new();
        for _ in 0..n_variables {
            variables.push(self.create_and_push_single_variable());
        }
        return variables;
    }
}

impl Deref for VariablesStack {
    type Target = Stack<Variable>;
    fn deref(&self) -> &Self::Target {
        &self.stack
    }
}

impl DerefMut for VariablesStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stack
    }
}

pub fn convert_vopcodes_to_lines(
    initial_stack: &VariablesStack,
    vopcodes: &[Vopcode],
) -> (VariablesStack, Vec<Line>) {
    let mut current_stack: VariablesStack = initial_stack.clone();
    let mut lines: Vec<Line> = Vec::new();

    for vopcode in vopcodes {
        let opcode: Opcode = vopcode.opcode;
        match opcode {
            Opcode::DUP { depth } => {
                let new_var: Variable = current_stack.create_single_variable();
                let assigned: Value = Value::Existing(current_stack.peek_at(depth - 1).clone());
                lines.push(Line::Assignement {
                    receiving_var: Some(new_var),
                    assigned_value: assigned,
                });
                current_stack.push(new_var);
            }
            Opcode::PUSH { item_size: _ } => {
                let pushed_value: U256 = vopcode.value.unwrap();
                let new_var: Variable = current_stack.create_single_variable();
                let assigned: Value = Value::Bytes(pushed_value);
                lines.push(Line::Assignement {
                    receiving_var: Some(new_var),
                    assigned_value: assigned,
                });
                current_stack.push(new_var);
            }
            Opcode::SWAP { depth } => {
                current_stack.swap(depth);
            }
            Opcode::POP | Opcode::JUMP => {
                current_stack.pop();
            }
            Opcode::JUMPDEST => (),
            Opcode::JUMPI => {
                current_stack.pop();
                lines.push(Line::If {
                    condition: Value::Existing(current_stack.pop()),
                });
            }
            _ => {
                let receiving_var: Option<Variable> = if opcode.stack_output() > 0 {
                    assert!(opcode.stack_output() == 1); // Opcodes only have 0 or 1 stack output in EVM
                    Some(current_stack.create_single_variable())
                } else {
                    None
                };
                let args = Value::from_vars(&current_stack.multi_pop(opcode.stack_input()));

                lines.push(Line::Assignement {
                    receiving_var,
                    assigned_value: Value::Calculation { opcode, args },
                });
                if let Some(var) = receiving_var {
                    current_stack.push(var);
                }
            }
        }
    }

    return (current_stack, lines);
}

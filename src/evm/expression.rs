use std::{fmt::Debug, hash::Hash};

use super::{context::Context, stack::Stack};
use crate::bytecode_reader::{opcode::Opcode, vopcode::Vopcode};
use primitive_types::U256;

pub trait Expression {
    // something we put on the stack
    fn apply_vopcode_on_context(context: &mut Context<Self>, vopcode: &Vopcode)
    where
        Self: Clone + Debug + Clone + Hash;
}

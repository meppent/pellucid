use super::expression::Expression;
use std::fmt::Debug;

#[derive(PartialEq, Clone, Hash)]
pub enum ExecutionState {
    RUNNING,
    REVERT,
    RETURN,
    SELFDESTRUCT,
    JUMP(Expression),              // program counter destination
    JUMPI(Expression, Expression), // program counter destination, condition
}

impl Debug for ExecutionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RUNNING => write!(f, "RUNNING"),
            Self::REVERT => write!(f, "REVERT"),
            Self::RETURN => write!(f, "RETURN"),
            Self::SELFDESTRUCT => write!(f, "SELFDESTRUCT"),
            Self::JUMP(arg0) => f.debug_tuple("JUMP").field(arg0).finish(),
            Self::JUMPI(arg0, arg1) => f.debug_tuple("JUMPI").field(arg0).field(arg1).finish(),
        }
    }
}

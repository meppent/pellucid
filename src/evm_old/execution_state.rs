use std::fmt::Debug;

use super::simple_expression::SimpleExpression;

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum ExecutionState {
    RUNNING,
    REVERT,
    RETURN,
    SELFDESTRUCT,
    JUMP {
        dest: SimpleExpression,
    },
    JUMPI {
        dest: SimpleExpression,
        condition: SimpleExpression,
    },
}

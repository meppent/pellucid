use std::fmt::Debug;

use super::expressions::expression::Expression;


#[derive(PartialEq, Clone, Debug, Hash)]
pub enum ExecutionState<Expr: Expression> {
    RUNNING,
    REVERT,
    RETURN,
    SELFDESTRUCT,
    JUMP { dest: Expr },
    JUMPI { dest: Expr, condition: Expr },
}
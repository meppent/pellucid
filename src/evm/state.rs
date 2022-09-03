use std::fmt::Debug;

use super::expression::Expression;

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum ExecutionState<Expr: Expression> {
    RUNNING,
    REVERT,
    RETURN,
    SELFDESTRUCT,
    JUMP { dest: Expr },
    JUMPI { dest: Expr, condition: Expr },
}

// impl<Expr: Expression> Debug for ExecutionState<Expr> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::RUNNING => write!(f, "RUNNING"),
//             Self::REVERT => write!(f, "REVERT"),
//             Self::RETURN => write!(f, "RETURN"),
//             Self::SELFDESTRUCT => write!(f, "SELFDESTRUCT"),
//             Self::JUMP(arg0) => f.debug_tuple("JUMP").field(arg0).finish(),
//             Self::JUMPI(arg0, arg1) => f.debug_tuple("JUMPI").field(arg0).field(arg1).finish(),
//         }
//     }
// }

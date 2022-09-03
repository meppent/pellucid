use crate::bytecode_reader::vopcode::Vopcode;

use super::{context::Context, expression::Expression, stack::Stack};
use primitive_types::U256;

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum SparseExpression {
    PUSH(U256),
    NONE,
}

impl PartialEq for Stack<SparseExpression> {
    fn eq(&self, other: &Self) -> bool {
        return self._get_data() == other._get_data();
    }
}

impl Expression for SparseExpression {
    fn apply_vopcode_on_context(context: &mut Context<Self>, vopcode: &Vopcode) {
        todo!()
    }
}

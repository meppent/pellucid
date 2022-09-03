use primitive_types::U256;

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum SimpleExpression {
    BYTES(U256),
    OTHER,
}

use super::incorporate_variables::{Line, Value, Variable};
use crate::execution_flow::execution_flow::{
    AccessContent, FunctionLabel, GetFunctionLabel, Length, Scope,
};

pub type VarScope = Scope<InstructionsWithVars, FunctionCallWithVars, FunctionReturnWithVars>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct InstructionsWithVars {
    pub lines: Vec<Line>,
}

impl Length for InstructionsWithVars {
    fn len(&self) -> usize {
        let mut length: usize = 0;
        for line in &self.lines {
            if line != &Line::Empty {
                length += 1;
            }
        }
        return length;
    }
}

#[derive(Clone, Eq, Debug)]
pub struct FunctionWithVars {
    pub label: FunctionLabel,
    pub input_vars: Vec<Variable>,
    pub n_outputs: usize,
    pub returns: bool, // true
    pub content: Vec<VarScope>,
}

impl GetFunctionLabel for FunctionWithVars {
    fn get_label(&self) -> FunctionLabel {
        return self.label;
    }
}

impl PartialEq for FunctionWithVars {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}
impl FunctionWithVars {
    pub fn n_parameters(&self) -> usize {
        return self.input_vars.len();
    }
}

impl AccessContent<InstructionsWithVars, FunctionCallWithVars, FunctionReturnWithVars>
    for FunctionWithVars
{
    fn get_content(&self) -> &Vec<VarScope> {
        return &self.content;
    }

    fn get_content_mut(&mut self) -> &mut Vec<VarScope> {
        return &mut self.content;
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FunctionCallWithVars {
    pub label: FunctionLabel,
    pub arguments: Vec<Value>,
    pub results: Vec<Variable>,
}

impl GetFunctionLabel for FunctionCallWithVars {
    fn get_label(&self) -> FunctionLabel {
        return self.label;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionReturnWithVars {
    pub label: FunctionLabel,
    pub returned_values: Vec<Value>,
}

impl GetFunctionLabel for FunctionReturnWithVars {
    fn get_label(&self) -> FunctionLabel {
        return self.label;
    }
}

impl VarScope {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Instructions(instructions) => instructions.lines.is_empty(),
            Self::Empty => true,
            _ => false,
        }
    }

    pub fn should_be_followed_by_condition_scope(&self) -> bool {
        if let Scope::Instructions(instructions) = self {
            if let Some(last_line) = instructions.lines.last() {
                return last_line.is_if();
            }
        }
        return false;
    }
}

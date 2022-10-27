use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    create_graph::block::Block,
    detect_functions::function::Function,
    tools::utils::{shift_text, usize_to_hex},
};

#[derive(Debug)]
pub struct SkeletonFunction<'a> {
    pub info: Function<'a>,
    pub instructions: Vec<SkeletonScope<'a>>,
}

#[derive(Debug)]
pub struct SkeletonIf<'a> {
    pub true_instructions: Vec<SkeletonScope<'a>>,
    pub false_instructions: Vec<SkeletonScope<'a>>,
}
#[derive(Debug)]
pub struct SkeletonJunction<'a> {
    pub starting_block: Block<'a>,
    pub instructions: Vec<SkeletonScope<'a>>,
}

#[derive(Debug)]
pub enum SkeletonScope<'a> {
    Function(Rc<RefCell<SkeletonFunction<'a>>>),
    LoopContinue { label: usize },
    Loop { label: usize },
    If(SkeletonIf<'a>),
    Junction(Rc<RefCell<SkeletonJunction<'a>>>),
    Block(Block<'a>),
    Panic,
}

impl<'a> SkeletonScope<'a> {
    pub fn to_string(&self) -> String {
        match self {
            SkeletonScope::Function(function_scope) => {
                let instructions_str: String =
                    instructions_to_string(&RefCell::borrow(function_scope).instructions);
                let mut res: String = String::new();
                res += &format!(
                    "def function_starting_at_{}:\n",
                    usize_to_hex(RefCell::borrow(function_scope).info.start.get_pc_start())
                );
                res += &shift_text(&instructions_str);
                return res;
            }
            SkeletonScope::LoopContinue { label } => format!("continue loop {}", label),
            SkeletonScope::Loop { label } => format!("start loop {}", label),
            SkeletonScope::If(if_scope) => {
                let true_instructions_str: String =
                    instructions_to_string(&if_scope.true_instructions);
                let false_instructions_str: String =
                    instructions_to_string(&if_scope.false_instructions);

                let mut res: String = String::new();
                res += "if:\n";
                res += &shift_text(&true_instructions_str);
                res += "else:\n";
                res += &shift_text(&false_instructions_str);
                res
            }
            SkeletonScope::Junction(junction_scope) => {
                let instructions_str: String =
                    instructions_to_string(&RefCell::borrow(junction_scope).instructions);
                let mut res: String = String::new();
                res += &format!(
                    "def {}:\n",
                    usize_to_hex(
                        RefCell::borrow(junction_scope)
                            .starting_block
                            .get_pc_start()
                    )
                );
                res += &shift_text(&instructions_str);
                res
            }
            SkeletonScope::Block(block) => {
                format!("-> execute block {}", usize_to_hex(block.get_pc_start()))
            }
            SkeletonScope::Panic => String::from("panic"),
        }
    }

    pub fn get_alias(&self) -> Option<String> {
        match self {
            SkeletonScope::Function(function_scope) => Some(format!(
                "function_starting_at_{}()",
                usize_to_hex(RefCell::borrow(function_scope).info.start.get_pc_start())
            )),
            SkeletonScope::LoopContinue { label: _ } => None,
            SkeletonScope::Loop { label: _ } => None,
            SkeletonScope::If(_) => None,
            SkeletonScope::Junction(junction_scope) => Some(format!(
                "junction_{}()",
                usize_to_hex(
                    RefCell::borrow(junction_scope)
                        .starting_block
                        .get_pc_start()
                )
            )),
            SkeletonScope::Block(_) => None,
            SkeletonScope::Panic => None,
        }
    }
}

pub fn instructions_to_string<'a>(instructions: &Vec<SkeletonScope<'a>>) -> String {
    let mut res: String = String::new();
    for instruction in instructions {
        if let Some(alias) = instruction.get_alias() {
            res.push_str(&alias);
        } else {
            res.push_str(&instruction.to_string());
        }
        res.push_str("\n");
    }

    return res;
}

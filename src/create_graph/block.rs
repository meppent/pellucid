use super::node::{Node, NodeRef};
use crate::{
    bytecode_reader::vopcode::Vopcode,
    tools::stack::Stack, create_blocks::{symbolic_block::SymbolicBlock}, evm_old::simple_expression::SimpleExpression,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Block<'a> {
    code: &'a [Vopcode],
    nodes: Vec<Rc<RefCell<Node<'a>>>>,
    pub symbolic_block: SymbolicBlock,
}

impl<'a> Block<'a> {
    pub fn new(code: &'a [Vopcode]) -> Self {
        return Block {
            code,
            nodes: vec![],
            symbolic_block: SymbolicBlock::new(),
        };
    }

    pub fn attach_symbolic_block(&mut self, symbolic_block: SymbolicBlock) {
        self.symbolic_block = symbolic_block;
    }
}

pub struct BlockRef<'a> {
    pub inner: Rc<RefCell<Block<'a>>>,
}

impl<'a> std::hash::Hash for BlockRef<'a> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_usize(self.get_pc_start());
        state.finish();
    }
}
impl<'a> PartialEq for BlockRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.get_pc_start() == other.get_pc_start();
    }
}
impl<'a> Eq for BlockRef<'a> {}

impl<'a> BlockRef<'a> {
    pub fn new(code: &'a [Vopcode]) -> Self {
        return BlockRef {
            inner: Rc::new(RefCell::new(Block::new(code))),
        };
    }

    pub fn clone(&self) -> Self {
        return BlockRef {
            inner: self.inner.clone(),
        };
    }

    pub fn add_node(&self, node: NodeRef<'a>) {
        self.inner.borrow_mut().nodes.push(node.inner);
    }

    pub fn nodes_count(&self) -> usize {
        return self.inner.borrow().nodes.len();
    }

    pub fn contains_initial_stack(&self, initial_stack: &Stack<SimpleExpression>) -> bool {
        for node in self.get_nodes() {
            if &node.get_initial_context().stack == initial_stack {
                return true;
            }
        }
        return false;
    }

    pub fn get_code(&self) -> &'a [Vopcode] {
        return self.inner.borrow().code;
    }

    pub fn get_pc_start(&self) -> usize {
        return self.get_code()[0].pc;
    }

    // pub fn get_delta(&self) -> isize {
    //     return self.inner.borrow().delta;
    // }

    // pub fn get_delta_min(&self) -> isize {
    //     return self.inner.borrow().delta_min;
    // }

    pub fn get_nodes(&self) -> Vec<NodeRef<'a>> {
        return self
            .inner
            .borrow()
            .nodes
            .iter()
            .map(|inner: &Rc<RefCell<Node<'a>>>| NodeRef {
                inner: inner.clone(),
            })
            .collect();
    }
}

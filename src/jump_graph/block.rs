use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use crate::bytecode_reader::opcode::Opcode;
use crate::bytecode_reader::{bytecode::Bytecode, vopcode::Vopcode};
use crate::evm::context::Context;
use crate::evm::stack::Stack;
use super::node::{Node, NodeRef};

#[derive(Debug)]
pub struct Block<'a> {
    code: &'a [Vopcode],
    delta: isize,
    delta_min: isize,
    nodes: Vec<Rc<RefCell<Node<'a>>>>,
}

pub struct BlockRef<'a>{
    inner: Rc<RefCell<Block<'a>>>
}

impl<'a> BlockRef<'a> {
    pub fn new(code: &'a [Vopcode], delta: isize, delta_min: isize) -> Self {
        return BlockRef{
            inner:Rc::new(
                RefCell::new(
                    Block{code, delta, delta_min, nodes: vec![]}
                )
            )
        };
    }

    pub fn clone(&self)->Self{
        return BlockRef { inner: self.inner.clone() }
    }

    pub fn add_node(&self, node: NodeRef<'a>){
       self.inner.borrow_mut().nodes.push(node.unwrap());
    }

    pub fn nodes_count(&self) -> usize {
        return self.inner.borrow().nodes.len();
    }

    pub fn contains_initial_stack(&self, initial_stack: &Stack)->bool{
        for node in self.get_nodes(){
            if node.get_initial_context().stack.equals_on_bytes(initial_stack){
                return true;
            }
        }
        return false;
    }

    pub fn get_code(&self)->&'a [Vopcode]{
        return self.inner.borrow().code;
    }

    pub fn get_pc_start(&self) -> usize{
        return self.get_code()[0].pc;
    }

    pub fn get_delta(&self)->isize{
        return self.inner.borrow().delta;
    }

    pub fn get_delta_min(&self)->isize{
        return self.inner.borrow().delta_min;
    }

    pub fn get_nodes(&self) -> Vec<NodeRef<'a>> {
        return self
            .inner
            .borrow()
            .nodes
            .iter()
            .map(|inner: &Rc<RefCell<Node<'a>>>| NodeRef::wrap(inner.clone()))
            .collect();
    }
}

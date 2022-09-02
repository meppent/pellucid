use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use crate::bytecode_reader::opcode::Opcode;
use crate::bytecode_reader::{bytecode::Bytecode, vopcode::Vopcode};
use crate::evm::context::Context;
use crate::evm::stack::Stack;
use super::block::{Block, BlockRef};


#[derive(Debug)]
pub struct Node<'a>{
    initial_context: Context,
    final_context: Context,
    block: Rc<RefCell<Block<'a>>>,
    parents: Vec<Rc<RefCell<Node<'a>>>>,
    children: Vec<Rc<RefCell<Node<'a>>>>,
}

impl <'a> std::hash::Hash for Node<'a> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        //state.write_usize(self.block.borrow().pc_start);
        state.finish();
    }
}

pub struct NodeRef<'a>{
    inner: Rc<RefCell<Node<'a>>>
}


impl<'a> NodeRef <'a> {
    pub fn new(block: Rc<RefCell<Block<'a>>>, initial_context: Context, final_context: Context) -> Self {
        return NodeRef{
            inner: Rc::new(
                RefCell::new(
                    Node {initial_context, final_context, block, parents: vec![], children: vec![]}
                )
            )
        };
    }

    pub fn create_with_neighbors(
        &self,
        block: Rc<RefCell<Block<'a>>>,
        initial_context: Context,
        final_context: Context,
        parents: Vec<NodeRef<'a>>,
        children: Vec<NodeRef<'a>>
    ) -> Self {
        let created = NodeRef::new(block, initial_context, final_context);
        for parent in parents{
            created.add_parent(parent);
        }
        for child in children{
            created.add_children(child);
        }
        return created
    }

    pub fn clone(&self) -> Self {
        return NodeRef {inner: self.inner.clone()}
    }

    pub fn get_initial_context(&self) -> Context {
        return self.inner.borrow().initial_context.clone();
    }

    pub fn get_final_context(&self) -> Context {
        return self.inner.borrow().final_context.clone();
    }
    
    pub fn get_children(&self) -> Vec<NodeRef<'a>> {
        return self
            .inner
            .borrow()
            .children
            .iter()
            .map(|inner: &Rc<RefCell<Node<'a>>>| NodeRef{inner: inner.clone()})
            .collect();
    }

    pub fn get_parents(&self) -> Vec<NodeRef<'a>> {
        return self
            .inner
            .borrow()
            .parents
            .iter()
            .map(|inner: &Rc<RefCell<Node<'a>>>| NodeRef{inner: inner.clone()})
            .collect();
    }
    
    pub fn add_parent(&self, parent: NodeRef<'a>) {
        self.inner.borrow_mut().parents.push(parent.clone().inner);
        parent.inner.borrow_mut().children.push(self.clone().inner);
    }

    pub fn add_children(&self, child: NodeRef<'a>) {
        self.inner.borrow_mut().children.push(child.clone().inner);
        child.inner.borrow_mut().parents.push(self.clone().inner);
    }

    pub fn wrap(inner: Rc<RefCell<Node<'a>>>)->Self{
        return NodeRef{inner};
    }

    pub fn unwrap(&self)->Rc<RefCell<Node<'a>>>{
        return self.inner.clone();
    }

}
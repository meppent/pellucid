use std::{cell::RefCell, rc::Rc};
use crate::bytecode_reader::opcode::Opcode;
use crate::bytecode_reader::{bytecode::Bytecode, vopcode::Vopcode};
use crate::evm::context::Context;
use crate::evm::stack::Stack;


#[derive(Debug)]
struct Block<'a> {
    code: &'a [Vopcode],
    delta: isize,
    delta_min: isize,
    nodes: Vec<Rc<RefCell<Node<'a>>>>,
}


struct BlockRef<'a>{
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
       self.inner.borrow_mut().nodes.push(node.inner);
    }

    pub fn number_of_nodes(&self) -> usize {
        return self.inner.borrow().nodes.len();
    }

    pub fn contains_initial_stack(&self, initial_stack: &Stack)->bool{
        for node in &self.inner.borrow().nodes{
            if node.borrow().initial_context.stack.equals_on_bytes(initial_stack){
                return true;
            }
        }
        return false;
    }

    //pub fn get_nodes(&self)

}

#[derive(Debug)]
struct Node<'a>{
    initial_context: Context,
    final_context: Context,
    block: Rc<RefCell<Block<'a>>>,
    parents: Vec<Rc<RefCell<Node<'a>>>>,
    children: Vec<Rc<RefCell<Node<'a>>>>,
}

struct NodeRef<'a>{
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

    pub fn clone(&self) -> Self {
        return NodeRef {inner: self.inner.clone()}
    }
    
    pub fn add_parent(self, parent: NodeRef<'a>) {
        self.inner.borrow_mut().parents.push(parent.clone().inner);
        parent.inner.borrow_mut().children.push(self.inner);
    }

    pub fn add_children(self, parent: NodeRef<'a>) {
        self.inner.borrow_mut().parents.push(parent.clone().inner);
        parent.inner.borrow_mut().children.push(self.inner);
    }

    pub fn get_block(&self) -> BlockRef<'a> {
        return BlockRef{inner: self.inner.borrow().block.clone()};
    }

}

struct Graph<'a> {
    pub blocks: Vec<Block<'a>>,
}

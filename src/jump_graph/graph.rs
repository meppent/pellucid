use std::collections::HashMap;
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

    pub fn nodes_count(&self) -> usize {
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

    pub fn get_nodes(&self) -> Vec<NodeRef<'a>> {
        return self
            .inner
            .borrow()
            .nodes
            .iter()
            .map(|inner: &Rc<RefCell<Node<'a>>>| NodeRef{inner: inner.clone()})
            .collect();
    }

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

    pub fn create_with_neighbors(&self, block: Rc<RefCell<Block<'a>>>, initial_context: Context, final_context: Context, parents: Vec<NodeRef<'a>>, children: Vec<NodeRef<'a>>) -> Self {
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

    pub fn get_block(&self) -> BlockRef<'a> {
        return BlockRef{inner: self.inner.borrow().block.clone()};
    }

}

struct Graph<'a> {
    pub blocks: HashMap<usize, BlockRef<'a>>,
}

impl <'a> Graph<'a> {

    pub fn new() -> Self {
        return Graph{blocks: HashMap::new()};
    }

    pub fn add_block(&mut self, block: BlockRef<'a>) {
        self.blocks[&block.code[0].pc] = block;
    }

    pub fn get_block(&self, index: usize) -> BlockRef<'a> {
        return BlockRef{inner: Rc::new(RefCell::new(self.blocks[index].clone()))};
    }
}
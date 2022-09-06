use super::{block::{Block, BlockRef}, simple_evm::{SimpleContext}};
use crate::{
    tools::utils::calculate_hash,
};
use std::{cell::RefCell, rc::Rc, fmt};

#[derive(Debug)]
pub struct Node<'a> {
    initial_context: SimpleContext,
    final_context: SimpleContext,
    block: BlockRef<'a>,
    parents: Vec<Rc<RefCell<Node<'a>>>>,
    children: Vec<Rc<RefCell<Node<'a>>>>,
}

pub struct NodeRef<'a> {
    pub inner: Rc<RefCell<Node<'a>>>,
}


impl<'a> std::hash::Hash for NodeRef<'a> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_usize(self.get_block().get_pc_start());
        state.write_u64(calculate_hash(&self.clone_initial_context()));
        state.finish();
    }
}

impl<'a> PartialEq for NodeRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        return calculate_hash(&self) == calculate_hash(&other);
    }
}
impl<'a> Eq for NodeRef<'a> {}

impl<'a> NodeRef<'a> {

    pub fn create_and_attach(
        block: BlockRef<'a>,
        initial_context: SimpleContext,
    ) -> Self {
        let final_context: SimpleContext = block.apply_on_simple_context(&initial_context);
        let node =  NodeRef {
            inner: Rc::new(RefCell::new(Node {
                initial_context: initial_context,
                final_context: final_context,
                block: block.clone(),
                parents: vec![],
                children: vec![],
            }))
        };

        block.add_node(node.clone());

        return node
    }


    pub fn clone(&self) -> Self {
        return NodeRef {
            inner: self.inner.clone(),
        };
    }

    pub fn clone_initial_context(&self) -> SimpleContext {
        return self.inner.borrow().initial_context.clone();
    }

    pub fn clone_final_context(&self) -> SimpleContext {
        return self.inner.borrow().final_context.clone();
    }

    pub fn get_block(&self) -> BlockRef<'a> {
        return self.inner.borrow().block.clone();
    }

    pub fn set_final_context(&self, final_context: SimpleContext){
        self.inner.borrow_mut().final_context = final_context;
    }

    pub fn get_children(&self) -> Vec<NodeRef<'a>> {
        return self
            .inner
            .borrow()
            .children
            .iter()
            .map(|inner: &Rc<RefCell<Node<'a>>>| NodeRef {
                inner: inner.clone(),
            })
            .collect();
    }

    pub fn get_parents(&self) -> Vec<NodeRef<'a>> {
        return self
            .inner
            .borrow()
            .parents
            .iter()
            .map(|inner: &Rc<RefCell<Node<'a>>>| NodeRef {
                inner: inner.clone(),
            })
            .collect();
    }

    //use only if nodes are already connected to blocks
    pub fn add_parent(&self, parent: NodeRef<'a>) {
        self.inner.borrow_mut().parents.push(parent.clone().inner);
        parent.inner.borrow_mut().children.push(self.clone().inner);
    }

    //use only if nodes are already connected to blocks
    pub fn add_child(&self, child: NodeRef<'a>) {
        self.inner.borrow_mut().children.push(child.clone().inner);
        child.inner.borrow_mut().parents.push(self.clone().inner);
    }



    
}

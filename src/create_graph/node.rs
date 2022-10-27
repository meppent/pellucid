use super::{block::Block, simple_evm::SimpleContext};
use crate::tools::utils::calculate_hash;
use std::collections::HashSet;
use std::fmt::{self, Debug};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct InnerNode<'a> {
    initial_context: SimpleContext,
    final_context: SimpleContext,
    block: Block<'a>,
    parents: Vec<Rc<RefCell<InnerNode<'a>>>>,
    children: Vec<Rc<RefCell<InnerNode<'a>>>>,
}

pub struct Node<'a> {
    pub inner: Rc<RefCell<InnerNode<'a>>>,
}

impl<'a> std::hash::Hash for Node<'a> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_u64(calculate_hash(&self.get_block()));
        state.write_u64(calculate_hash(&self.clone_initial_context()));
        state.finish();
    }
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        return calculate_hash(&self) == calculate_hash(&other);
    }
}
impl<'a> Eq for Node<'a> {}

impl<'a> Node<'a> {
    pub fn create_and_attach(block: Block<'a>, initial_context: SimpleContext) -> Self {
        let final_context: SimpleContext = block.apply_on_simple_context(&initial_context);
        let node = Node {
            inner: Rc::new(RefCell::new(InnerNode {
                initial_context: initial_context,
                final_context: final_context,
                block: block.clone(),
                parents: vec![],
                children: vec![],
            })),
        };

        block.add_node(node.clone());

        return node;
    }

    // pub fn clone(&self) -> Self {
    //     return Node {
    //         inner: self.inner.clone(),
    //     };
    // }

    pub fn clone_initial_context(&self) -> SimpleContext {
        return self.inner.borrow().initial_context.clone();
    }

    pub fn clone_final_context(&self) -> SimpleContext {
        return self.inner.borrow().final_context.clone();
    }

    pub fn get_block(&self) -> Block<'a> {
        return self.inner.borrow().block.clone();
    }

    pub fn set_final_context(&self, final_context: SimpleContext) {
        self.inner.borrow_mut().final_context = final_context;
    }

    pub fn is_orphan(&self) -> bool {
        // concerns the first node of the graph + some nodes at the beginning of loops (due to cut connections)
        return self.get_parents().is_empty();
    }

    pub fn get_children(&self) -> Vec<Node<'a>> {
        return self
            .inner
            .borrow()
            .children
            .iter()
            .map(|inner: &Rc<RefCell<InnerNode<'a>>>| Node {
                inner: inner.clone(),
            })
            .collect();
    }

    pub fn get_parents(&self) -> Vec<Node<'a>> {
        return self
            .inner
            .borrow()
            .parents
            .iter()
            .map(|inner: &Rc<RefCell<InnerNode<'a>>>| Node {
                inner: inner.clone(),
            })
            .collect();
    }

    pub fn get_block_parents(&self) -> HashSet<Block<'a>> {
        return self
            .get_parents()
            .iter()
            .map(|n: &Node| n.get_block())
            .collect();
    }

    pub fn get_block_children(&self) -> HashSet<Block<'a>> {
        return self
            .get_children()
            .iter()
            .map(|n: &Node| n.get_block())
            .collect();
    }

    //use only if nodes are already connected to blocks
    pub fn add_parent(&self, parent: Node<'a>) {
        self.inner.borrow_mut().parents.push(parent.clone().inner);
        parent.inner.borrow_mut().children.push(self.clone().inner);
    }

    //use only if nodes are already connected to blocks
    pub fn add_child(&self, child: Node<'a>) {
        self.inner.borrow_mut().children.push(child.clone().inner);
        child.inner.borrow_mut().parents.push(self.clone().inner);
    }

    pub fn remove_child(&self, child: Node<'a>) {
        assert!(self.get_children().contains(&child));
        for (index, _child) in self.get_children().iter().enumerate() {
            if _child == &child {
                self.inner.borrow_mut().children.remove(index);
                return;
            }
        }
        panic!("Couldn't remove child {:?} of {:?}", child, self);
    }

    pub fn remove_parent(&self, parent: Node<'a>) {
        assert!(self.get_parents().contains(&parent));
        for (index, _parent) in self.get_parents().iter().enumerate() {
            if _parent == &parent {
                self.inner.borrow_mut().parents.remove(index);
                return;
            }
        }
        panic!("Couldn't remove parent {:?} of {:?}", parent, self);
    }

    pub fn get_index_in_block(&self) -> usize {
        //debug only
        let mut index: usize = 0;
        for (neighbour_index, neighbour) in self.get_block().get_nodes().into_iter().enumerate() {
            if &neighbour == self {
                index = neighbour_index;
                break;
            }
        }
        return index;
    }

    pub fn switch_block(&self, new_block: &Block<'a>) {
        self.get_block().remove_node(&self);
        RefCell::borrow_mut(&self.inner).block = new_block.clone();
        new_block.add_node(self.clone());
    }
}

impl<'a> Debug for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Block")
            .field("block", &format!("{:?}", self.get_block()))
            .field("index", &self.get_index_in_block())
            .field("n_parents", &self.get_parents().len())
            .field("n_children", &self.get_children().len())
            .finish()
    }
}
// Warning: This is not a 'real' clone
impl<'a> Clone for Node<'a> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

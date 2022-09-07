use primitive_types::U256;

use super::{node::{Node, NodeRef}, simple_evm::{SimpleStack, SimpleContext}};
use crate::{
    bytecode_reader::{vopcode::Vopcode, opcode::Opcode}, create_blocks::{symbolic_block::SymbolicBlock, symbolic_expression::{StackExpression, Effect, SymbolicExpression}}, create_graph::simple_evm::{State, SimpleStackExpression}, tools::stack::Stack,
};
use std::{cell::RefCell, rc::Rc, fmt};
#[derive(Default)]
pub struct Block<'a> {
    pub code: &'a [Vopcode],
    nodes: Vec<Rc<RefCell<Node<'a>>>>,
    pub symbolic_block: Rc<SymbolicBlock>,
}

impl<'a> fmt::Debug for Block<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Block")
         .field("code", &"code")
         .field("node_n", &self.nodes.len())
         .finish()
    }
}

impl<'a> Block<'a> {
    pub fn new(code: &'a [Vopcode], symbolic_block: Rc<SymbolicBlock>) -> Self {
        return Block {
            code,
            nodes: vec![],
            symbolic_block,
        };
    }
}

#[derive(Debug)]
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
    pub fn new(code: &'a [Vopcode], symbolic_block: Rc<SymbolicBlock>) -> Self {
        return BlockRef {
            inner: Rc::new(RefCell::new(Block::new(code, symbolic_block))),
        };
    }

    pub fn from_block(block: Block)->BlockRef{
        return BlockRef { inner: Rc::new(RefCell::new(block)) };
    }

    pub fn clone(&self) -> Self {
        return BlockRef {
            inner: self.inner.clone(),
        };
    }


    pub fn add_node(&self, node: NodeRef<'a>) {
        self.inner.borrow_mut().nodes.push(node.inner);
    }

    pub fn get_symbolic_block(&self) -> Rc<SymbolicBlock>{
        return self.inner.borrow().symbolic_block.clone();
    }

    pub fn nodes_count(&self) -> usize {
        return RefCell::borrow(&self.inner).nodes.len();
    }

    pub fn get_node_starting_with(&self, initial_context: &SimpleContext) -> Option<NodeRef<'a>> {
        for node in self.get_nodes() {
            if &node.clone_initial_context() == initial_context {
                return Some(node.clone());
            }
        }
        return None;
    }

    pub fn get_code(&self) -> &'a [Vopcode] {
        return RefCell::borrow(&self.inner).code;
    }

    pub fn get_pc_start(&self) -> usize {
        return self.get_code()[0].pc;
    }
    
    pub fn get_pc_end(&self) -> usize {
        return self.get_code()[self.get_code().len() - 1].pc;
    }

    pub fn get_nodes(&self) -> Vec<NodeRef<'a>> {
        return RefCell::borrow(&self.inner)
            .nodes
            .iter()
            .map(|inner: &Rc<RefCell<Node<'a>>>| NodeRef {
                inner: inner.clone(),
            })
            .collect();
    }

    pub fn get_n_args(&self)->usize{
        return  RefCell::borrow(&self.inner).symbolic_block.n_args;
    }

    pub fn get_next_pc_start(&self) -> usize {
        return self.get_code()[self.get_code().len() - 1].get_next_pc();
    }

    pub fn final_effect(&self)-> Option<Rc<Effect>> {
        return RefCell::borrow(&self.inner).symbolic_block.final_effect();
    }

    pub fn apply_on_simple_context(&self, initial_context: &SimpleContext) -> SimpleContext {
        // return the resulting stack + the list of the next pc destinations
        assert!(initial_context.state == State::RUNNING); // I want to delete this
        let mut final_context: SimpleContext = initial_context.clone();

        if self.get_n_args() > initial_context.stack.len(){
            final_context.state = State::STOP;
            return final_context;
        }

        let mut args: Vec<SimpleStackExpression> = vec![];
        for _ in 0..self.get_n_args(){
            args.push(final_context.stack.pop());
        }

        for symbolic_expr in self.get_symbolic_block().stack.iter() {
            match symbolic_expr.stack_expression {
                StackExpression::BYTES(value) => final_context.stack.push(SimpleStackExpression::BYTES(value)),
                StackExpression::ARG(index) => final_context.stack.push(args[index - 1].clone()),
                StackExpression::COMPOSE(_,_) => final_context.stack.push(SimpleStackExpression::OTHER),
            }
        }

        final_context.state = self.compute_final_state(self.final_effect(), args);

        return final_context;
    }

    pub fn compute_final_state(&self, final_effect: Option<Rc<Effect>>, args: Vec<SimpleStackExpression>) -> State {
        match final_effect {
            None => { return State::RUNNING; },
            Some(final_effect) => {
                if [Opcode::JUMPI, Opcode::JUMP].contains(&final_effect.opcode) {
                    let mut destinations: Vec<usize> = Vec::new();
                    if final_effect.opcode == Opcode::JUMPI { destinations.push(self.get_next_pc_start()) }

                    match final_effect.symbolic_exprs[0].stack_expression {
                        
                        StackExpression::COMPOSE(_, _) => panic!("JUMP destination is not a constant"),
                        StackExpression::BYTES(dest) => destinations.push(dest.as_usize()),
                        StackExpression::ARG(value) => 
                            match args[value - 1]{
                                SimpleStackExpression::BYTES(dest) => destinations.push(dest.as_usize()),
                                _ => panic!("JUMP destination is not a constant")
                            },
                    }
                    return State::JUMP(destinations);
                } 
                else if final_effect.opcode.is_exiting() { return State::STOP; }
                else { return State::RUNNING;}
                }
            }
        }
    }
        
    



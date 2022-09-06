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
         .field("code", &self.code)
         .field("node_n", &self.nodes.len())
         .finish()
    }
}

impl<'a> Block<'a> {
    pub fn new(code: &'a [Vopcode]) -> Self {
        return Block {
            code,
            nodes: vec![],
            symbolic_block: Rc::new(SymbolicBlock::new()),
        };
    }

    pub fn attach_symbolic_block(&mut self, symbolic_block: Rc<SymbolicBlock>) {
        self.symbolic_block = symbolic_block;
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
    pub fn new(code: &'a [Vopcode]) -> Self {
        return BlockRef {
            inner: Rc::new(RefCell::new(Block::new(code))),
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

    pub fn contains_initial_context(&self, initial_context: &SimpleContext) -> bool {
        for node in self.get_nodes() {
            if &node.get_initial_context() == initial_context {
                return true;
            }
        }
        return false;
    }

    pub fn get_code(&self) -> &'a [Vopcode] {
        return RefCell::borrow(&self.inner).code;
    }

    pub fn get_pc_start(&self) -> usize {
        return self.get_code()[0].pc;
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
      
        if let Some(final_effect) = self.final_effect() {

            match &*final_effect {
                
                Effect::COMPOSE(Opcode::JUMP, dest) => {
                    
                    assert!(dest.len() == 1, "JUMP without good number of arguments");

                    match dest[0].stack_expression {
                        StackExpression::BYTES(dest) => {
                            final_context.state = State::JUMP(vec![dest.as_usize()]);
                        },
                        StackExpression::ARG(value) => {
                            match args[value - 1]{
                                SimpleStackExpression::BYTES(dest) => {
                                    final_context.state = State::JUMP(vec![dest.as_usize()]);
                                },
                                _ => {
                                    panic!("JUMP destination is not a constant")
                                }
                            }
                        },
                        _ => { panic!("JUMP destination is not a constant") }
                    }
                },

                Effect::COMPOSE(Opcode::JUMPI, dest) => {
                
                    assert!(dest.len() == 2, "JUMP without good number of arguments");
                    let mut dests_bytes: Vec<usize> = vec![self.get_next_pc_start()];

                    match &dest[0].stack_expression {
                        StackExpression::BYTES(dest) => {
                            dests_bytes.push(dest.as_usize());
                        },
                        StackExpression::ARG(value) => {
                            match args[value - 1]{
                                SimpleStackExpression::BYTES(dest) => { dests_bytes.push(dest.as_usize()); },
                                _ => { panic!("JUMP destination is not a constant") }
                            }
                        },
                        _ => { panic!("JUMP destination is not a constant") }
                    }

                    

                    final_context.state = State::JUMP(dests_bytes);
                },
              
                Effect::COMPOSE(opcode, _) => {
                    if opcode.is_exiting() {
                        final_context.state = State::STOP;
                    } else {
                        final_context.state = State::RUNNING;
                    }
                }
            }
        }
        return final_context;
    }




}

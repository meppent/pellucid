use super::{node::Node, simple_evm::SimpleContext};
use crate::{
    bytecode_reader::{opcode::Opcode, vopcode::Vopcode},
    create_blocks::{
        symbolic_block::SymbolicBlock,
        symbolic_expression::{Effect, StackExpression},
    },
    create_graph::simple_evm::{SimpleStackExpression, State},
    tools::utils::{calculate_hash, usize_to_hex},
};
use std::{cell::RefCell, collections::HashSet, fmt, fmt::Debug, rc::Rc};

pub struct InnerBlock<'a> {
    code: &'a [Vopcode],
    nodes: Vec<Node<'a>>,
    symbolic_block: SymbolicBlock,
    duplication_info: Option<(usize, Block<'a>)>, // (duplication index, ancestor)
}

impl<'a> InnerBlock<'a> {
    pub fn new(
        code: &'a [Vopcode],
        symbolic_block: SymbolicBlock,
        duplication_info: Option<(usize, Block<'a>)>,
    ) -> Self {
        return InnerBlock {
            code,
            nodes: Vec::new(),
            symbolic_block,
            duplication_info,
        };
    }
}

pub struct Block<'a> {
    pub inner: Rc<RefCell<InnerBlock<'a>>>,
}

impl<'a> std::hash::Hash for Block<'a> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_usize(self.get_pc_start());
        if let Some(previous_duplicate) = self.get_duplication_info() {
            state.write_u64(calculate_hash(&previous_duplicate));
        }
        state.finish();
    }
}
impl<'a> PartialEq for Block<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.get_pc_start() == other.get_pc_start()
            && self.get_duplication_info() == other.get_duplication_info();
    }
}
impl<'a> Eq for Block<'a> {}

impl<'a> Block<'a> {
    pub fn new(code: &'a [Vopcode], duplication_info: Option<(usize, Block<'a>)>) -> Block<'a> {
        let symbolic_block: SymbolicBlock = SymbolicBlock::from(code);
        return Block {
            inner: Rc::new(RefCell::new(InnerBlock::new(
                code,
                symbolic_block,
                duplication_info,
            ))),
        };
    }

    pub fn from_block(block: InnerBlock) -> Block {
        return Block {
            inner: Rc::new(RefCell::new(block)),
        };
    }

    pub fn add_node(&self, node: Node<'a>) {
        self.inner.borrow_mut().nodes.push(node);
    }

    pub fn clone_symbolic_block(&self) -> SymbolicBlock {
        return self.inner.borrow().symbolic_block.clone();
    }

    pub fn nodes_count(&self) -> usize {
        return RefCell::borrow(&self.inner).nodes.len();
    }

    pub fn get_node_starting_with(&self, initial_context: &SimpleContext) -> Option<Node<'a>> {
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

    pub fn get_nodes(&self) -> Vec<Node<'a>> {
        return RefCell::borrow(&self.inner).nodes.clone();
    }

    pub fn get_child_blocks(&self) -> HashSet<Block<'a>> {
        let mut child_blocks: HashSet<Block<'a>> = HashSet::new();
        for node in self.get_nodes() {
            for child_node in node.get_children() {
                child_blocks.insert(child_node.get_block());
            }
        }
        return child_blocks;
    }

    pub fn get_child_pc_starts(&self) -> HashSet<usize> {
        return self
            .get_child_blocks()
            .iter()
            .map(|b| b.get_pc_start())
            .collect();
    }

    pub fn get_parent_pc_starts(&self) -> HashSet<usize> {
        return self
            .get_parent_blocks()
            .iter()
            .map(|b| b.get_pc_start())
            .collect();
    }

    pub fn get_parent_blocks(&self) -> HashSet<Block<'a>> {
        let mut parent_blocks: HashSet<Block<'a>> = HashSet::new();
        for node in self.get_nodes() {
            for parent_node in node.get_parents() {
                parent_blocks.insert(parent_node.get_block());
            }
        }
        return parent_blocks;
    }

    pub fn get_n_args(&self) -> usize {
        return RefCell::borrow(&self.inner).symbolic_block.n_args;
    }

    pub fn get_next_pc_start(&self) -> usize {
        return self.get_code()[self.get_code().len() - 1].get_next_pc();
    }

    pub fn has_some_children(&self) -> bool {
        for node in &RefCell::borrow(&self.inner).nodes {
            if node.get_children().len() > 0 {
                return true;
            }
        }
        return false;
    }

    pub fn final_effect(&self) -> Option<Rc<Effect>> {
        return RefCell::borrow(&self.inner).symbolic_block.final_effect();
    }

    pub fn is_dead_end(&self) -> bool {
        return self.get_child_blocks().is_empty();
    }

    pub fn apply_on_simple_context(&self, initial_context: &SimpleContext) -> SimpleContext {
        // return the resulting stack + the list of the next pc destinations
        assert!(initial_context.state == State::RUNNING); // I want to delete this
        let mut final_context: SimpleContext = initial_context.clone();

        if self.get_n_args() > initial_context.stack.len() {
            final_context.state = State::STOP;
            return final_context;
        }

        let mut args: Vec<SimpleStackExpression> = vec![];
        for _ in 0..self.get_n_args() {
            args.push(final_context.stack.pop());
        }

        for symbolic_expr in self.clone_symbolic_block().symbolic_expressions.iter() {
            match symbolic_expr.stack_expression {
                StackExpression::BYTES(value) => final_context
                    .stack
                    .push(SimpleStackExpression::BYTES(value)),
                StackExpression::ARG(index) => final_context.stack.push(args[index - 1].clone()),
                StackExpression::COMPOSE(_, _) => {
                    final_context.stack.push(SimpleStackExpression::OTHER)
                }
            }
        }

        final_context.state = self.compute_final_state(self.final_effect(), args);

        return final_context;
    }

    pub fn compute_final_state(
        &self,
        final_effect: Option<Rc<Effect>>,
        args: Vec<SimpleStackExpression>,
    ) -> State {
        match final_effect {
            None => {
                return State::RUNNING;
            }
            Some(final_effect) => {
                if final_effect.opcode.is_jump() {
                    let mut destinations: Vec<usize> = Vec::new();
                    if final_effect.opcode == Opcode::JUMPI {
                        destinations.push(self.get_next_pc_start())
                    }
                    let final_expression: &StackExpression =
                        &final_effect.symbolic_expressions[0].stack_expression;
                    match final_expression {
                        StackExpression::COMPOSE(_, _) => {
                            /*
                            In case the computation of the jumpdests involves bytes from previous blocks, we could also add another type in SimpleStackExpression:
                            COMPUTED_BYTES(U256) where we put the result of all the arithmetical operations wa manage to perform (when all the inputs are BYTES or
                            COMPUTED_BYTES). We could continue to compare stacks based on BYTES (and not COMPUTED_BYTES), but we could a COMPUTED_BYTES when it's a
                            jump destination. Issue: In the latter case, how to handle the previous comparisons that didn't take into account this COMPUTED_BYTES
                            while it should have been the case, because it is in fact a jump destination. We should probably go back in time, at the moment where
                            the involved COMPUTED_BYTES is beeing computed... Hard to implement
                            */
                            destinations.push(
                                final_expression
                                    .compute_value()
                                    .expect("cannot compute jump dest")
                                    .as_usize(),
                            );
                            log::info!("Jumpdest required a computation");
                        }
                        StackExpression::BYTES(dest) => destinations.push(dest.as_usize()),
                        StackExpression::ARG(value) => match args[value - 1] {
                            SimpleStackExpression::BYTES(dest) => {
                                destinations.push(dest.as_usize())
                            }
                            _ => panic!("JUMP destination is not a constant"),
                        },
                    }
                    return State::JUMP(destinations);
                } else if final_effect.opcode.is_exiting() {
                    return State::STOP;
                } else {
                    return State::RUNNING;
                }
            }
        }
    }

    pub fn has_deterministic_child_blocks(&self) -> bool {
        let final_vopcode: Vopcode = self.get_code()[self.get_code().len() - 1];
        if !final_vopcode.opcode.is_jump() {
            return true;
        }
        if self.get_code().len() <= 2 && self.get_code()[self.get_code().len() - 2].opcode.is_push()
        {
            return true;
        }
        if final_vopcode.opcode == Opcode::JUMPI {
            return self.get_child_blocks().len() == 2;
        }
        if final_vopcode.opcode == Opcode::JUMP {
            return self.get_child_blocks().len() == 1;
        }
        // TODO handle edge case of a jumpi where the jump dest equals next pc
        unreachable!();
    }

    pub fn get_next_conditional_dests(&self) -> Option<(Block<'a>, Block<'a>)> {
        // (block at the PC given in parameter, block right after the JUMPI)
        if self.get_child_blocks().len() != 2 || !self.has_deterministic_child_blocks() {
            return None;
        }
        let last_vopcode: &Vopcode = self.get_code().last().unwrap();
        let child_blocks: Vec<Block> = self.get_child_blocks().iter().cloned().collect();
        if child_blocks[0].get_pc_start() == last_vopcode.get_next_pc() {
            return Some((child_blocks[1].clone(), child_blocks[0].clone()));
        } else {
            assert!(child_blocks[1].get_pc_start() == last_vopcode.get_next_pc());
            return Some((child_blocks[0].clone(), child_blocks[1].clone()));
        }
    }

    pub fn get_all_nodes_from_many_blocks(blocks: &HashSet<Block<'a>>) -> HashSet<Node<'a>> {
        let mut all_nodes: HashSet<Node<'a>> = HashSet::new();
        for block in blocks {
            all_nodes.extend(block.get_nodes());
        }
        return all_nodes;
    }

    pub fn get_all_orphan_nodes(blocks: &HashSet<Block<'a>>) -> HashSet<Node<'a>> {
        return Self::get_all_nodes_from_many_blocks(blocks)
            .iter()
            .filter(|n| n.is_orphan())
            .cloned()
            .collect();
    }

    pub fn get_duplication_info(&self) -> Option<(usize, Block<'a>)> {
        return RefCell::borrow(&self.inner).duplication_info.clone();
    }

    pub fn remove_node(&self, node: &Node<'a>) {
        let index: usize = RefCell::borrow(&self.inner)
            .nodes
            .iter()
            .position(|n| *n == *node)
            .unwrap();
        RefCell::borrow_mut(&self.inner).nodes.remove(index);
        assert!(!self.get_nodes().contains(node));
    }
}

impl<'a> Debug for Block<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Block")
            .field("block", &usize_to_hex(self.get_pc_start()))
            .field("duplication info", &self.get_duplication_info())
            .field("n_nodes", &self.get_nodes().len())
            .finish()
    }
}

// Warning: This is not a 'real' clone
impl<'a> Clone for Block<'a> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

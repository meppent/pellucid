use std::collections::HashMap;

use itertools::Itertools;

use crate::bytecode_reader::opcode::Opcode;
use crate::bytecode_reader::{bytecode::Bytecode, vopcode::Vopcode};
use crate::evm::context::Context;


use crate::utils::{remove_values_where};

use super::block::{Block, Location, Position, STOP_OPCODES};

pub struct ConnectedBlock<'a> {
    pub block: Block<'a>,
    pub links: HashMap<usize, HashMap<Position, Vec<Location>>>, // context index => {UP => list of its parents locations, DOWN => list of its children locations}
}

impl<'a> ConnectedBlock<'a> {
    pub fn new(block: Block<'a>) -> Self {
        return ConnectedBlock {
            block,
            links: HashMap::new(),
        };
    }
}

pub struct BlockSet<'a> {
    connected_blocks: HashMap<usize, ConnectedBlock<'a>>, // pc_start => connected block
}

impl<'a> BlockSet<'a> {
    pub fn contains_block_at(&self, pc: usize) -> bool {
        return self.connected_blocks.contains_key(&pc);
    }

    pub fn get_connected_block_mut(&mut self, pc_start: usize) -> &mut ConnectedBlock<'a> {
        return self.connected_blocks.get_mut(&pc_start).unwrap();
    }

    pub fn get_block_mut(&mut self, pc_start: usize) -> &mut Block<'a> {
        return &mut self.get_connected_block_mut(pc_start).block;
    }

    pub fn get_blocks(&self) -> Vec<Block<'a>> {
        return self
            .connected_blocks
            .values()
            .map(|connected_block: &ConnectedBlock| connected_block.block.clone())
            .collect::<Vec<Block<'a>>>();
    }

    pub fn get_pc_end_of_block(&self, pc_start: usize) -> usize {
        return self.connected_blocks[&pc_start].block.get_pc_end();
    }

    pub fn get_all_pc_starts(&self) -> Vec<usize> {
        return self.connected_blocks.keys().cloned().collect_vec();
    }

    pub fn get_edges(&self) -> Vec<(usize, usize)> {
        let mut edges: Vec<(usize, usize)> = vec![]; // (pc_start origin, pc_start dest)
        for (_, connected_block) in &self.connected_blocks {
            let origin_pc_start: usize = connected_block.block.get_pc_start();
            for (_, sub_links) in &connected_block.links {
                for dest_location in &sub_links[&Position::FINAL] {
                    let dest_pc_start: usize = dest_location.pc_start;
                    edges.push((origin_pc_start, dest_pc_start));
                }
            }
        }
        return edges;
    }

    pub fn new(bytecode: &'a Bytecode) -> Self {
        let mut block_set: BlockSet = BlockSet {
            connected_blocks: HashMap::new(),
        };
        block_set.find_blocks(bytecode);
        block_set.extend(Context::new(), 0);
        return block_set;
    }

    fn find_blocks(&mut self, bytecode: &'a Bytecode) {
        let mut pc_start: Option<usize> = Some(0);
        let mut previous_opcode: Option<Opcode> = None;
        let mut delta: isize = 0;
        let mut delta_min: isize = 0;

        for vopcode in bytecode.iter(0, bytecode.get_last_pc()) {
            let opcode: Opcode = vopcode.opcode;
            let pc: usize = vopcode.pc;
            let next_opcode: Option<Opcode> = if let Some(next_pc) = vopcode.get_next_pc() {
                Some(bytecode.get_vopcode_at(next_pc).opcode)
            } else {
                None
            };
            if pc_start == None
                && (opcode == Opcode::JUMPDEST || previous_opcode == Some(Opcode::JUMPI))
            {
                // start a new block
                pc_start = Some(pc);
            }
            if pc_start != None {
                delta += opcode.delta();
                if delta < delta_min {
                    delta_min = delta;
                }
                if (STOP_OPCODES.contains(&opcode) || next_opcode == Some(Opcode::JUMPDEST))
                    || vopcode.is_last
                {
                    // end block
                    delta = 0;
                    delta_min = 0;

                    self.connected_blocks.insert(
                        pc_start.unwrap(),
                        ConnectedBlock::new(Block::new(
                            bytecode.slice_code(pc_start.unwrap(), pc),
                            delta,
                            delta_min,
                        )),
                    );
                    pc_start = None;
                }
            }
            previous_opcode = Some(opcode);
        }
    }

    fn connect(&mut self, location_to_connect: Location, dest_location: Location) {
        let links: &mut HashMap<usize, HashMap<Position, Vec<Location>>> = &mut self
            .get_connected_block_mut(location_to_connect.pc_start)
            .links;

        if !links.contains_key(&location_to_connect.context_index) {
            links.insert(
                location_to_connect.context_index,
                HashMap::from([(Position::INITIAL, vec![]), (Position::FINAL, vec![])]),
            );
        }
        links
            .get_mut(&location_to_connect.context_index)
            .unwrap()
            .get_mut(&location_to_connect.position)
            .unwrap()
            .push(dest_location);
    }

    fn connect_both(&mut self, location_0: Location, location_1: Location) {
        self.connect(location_0, location_1);
        self.connect(location_1, location_0);
    }

    fn extend(&mut self, initial_context: Context, pc_start: usize) {
        let block: &mut Block = self.get_block_mut(pc_start);
        if block.contains_initial_context(&initial_context) {
            return;
        }
        let (origin_context_index, final_context, mut next_dests): (usize, Context, Vec<usize>) =
            block.add_initial_context(initial_context);
        let last_vopcode: Vopcode = block.get_last_vopcode();
        remove_values_where(&mut next_dests, |jump_dest: &usize| {
            !self.contains_block_at(*jump_dest)
                || !self.connected_blocks[jump_dest]
                    .block
                    .is_jumpable_from(last_vopcode)
        }); // remove potential invalid destinations
        let dest_initial_context: Context = final_context.clean_state();
        let origin_location: Location = Location {
            pc_start,
            context_index: origin_context_index,
            position: Position::FINAL,
        };
        for next_dest in next_dests {
            let dest_block: &mut Block = self.get_block_mut(next_dest);
            let dest_context_index: usize =
                dest_block.get_index_of_incomming_initial_context(&dest_initial_context);
            let dest_location: Location = Location {
                pc_start: next_dest,
                context_index: dest_context_index,
                position: Position::INITIAL,
            };
            self.connect_both(origin_location, dest_location);
            self.extend(dest_initial_context.clone(), next_dest);
        }
    }
}
mod test{
    use crate::{utils::{tests::Contract, write_file}, bytecode_reader::bytecode::Bytecode, jump_graph::{display::draw, gml::to_gml}};
    use super::BlockSet;

#[test]
fn test_gml() {
    let contract: Contract = Contract::SIMPLE_CONTRACT;
    let bytecode: Bytecode = Bytecode::from(&contract.get_bytecode());

    let block_set: BlockSet = BlockSet::new(&bytecode);
    let gml: String = to_gml(&block_set);
    if gml != contract.get_gml(){
        write_file("temp_target.gml", &contract.get_gml());
        write_file("temp_current.gml", &gml);
        panic!("the gml is invalid, temp files were generated so you can see the diffs");
    }
    
}
#[test]
fn test_graph_drawing() {
    let contract: Contract = Contract::SIMPLE_CONTRACT;
    let bytecode: Bytecode = Bytecode::from(&contract.get_bytecode());
    let block_set: BlockSet = BlockSet::new(&bytecode);
    let graph_drawing: String = draw(&block_set, &bytecode);
    let graph_drawing_ref : String = contract.get_graph_drawing();

    if graph_drawing != contract.get_graph_drawing(){
        write_file("temp_ref_drawing.txt", &graph_drawing_ref);
        write_file("temp_current_drawing.txt", &graph_drawing);
        panic!("drawings differ, temp files were generated so you can see the diffs");
    }
}}

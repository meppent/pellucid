use pellucid::{bytecode_reader::bytecode, utils::read_file};

pub enum Contract {
    SIMPLE_CONTRACT,
}

impl Contract {
    fn get_path(&self) -> String {
        let mut res: String = String::from("./tests/contracts/");
        res.push_str(match self {
            Self::SIMPLE_CONTRACT => "simple_contract/",
        });
        return res;
    }

    pub fn get_bytecode(&self) -> String {
        let mut bytecode_path: String = self.get_path();
        bytecode_path.push_str("bytecode.txt");
        return read_file(&bytecode_path);
    }

    pub fn get_opcodes(&self) -> String {
        let mut opcodes_path: String = self.get_path();
        opcodes_path.push_str("opcodes.txt");
        return read_file(&opcodes_path);
    }

    pub fn get_graph_drawing(&self) -> String {
        let mut opcodes_path: String = self.get_path();
        opcodes_path.push_str("graph_drawing.txt");
        return read_file(&opcodes_path);
    }
}

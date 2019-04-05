use Opcodes;

#[derive(Clone, RustcEncodable)]
pub struct Instruction {
    opcode: Opcodes::Opcodes,
    params: Vec<String>
}

impl Instruction {
    pub fn new(opcode: Opcodes::Opcodes, params: Vec<String>) -> Instruction {
        Instruction {
            opcode: opcode,
            params: params
        }
    }
    pub fn set_opcode(&mut self, opcode: Opcodes::Opcodes) {
        self.opcode = opcode;
    }
    pub fn set_params(&mut self, params: Vec<String>) {
        self.params = params;
    }
    pub fn add_param(&mut self, param: String) {
        self.params.push(param);
    }
}
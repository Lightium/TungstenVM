use Opcodes;

#[derive(Clone, RustcDecodable)]
pub struct Instruction {
    pub opcode: Opcodes::Opcodes,
    pub params: Vec<String>
}

impl Instruction {
    pub fn new() -> Instruction {
        Instruction {
            opcode: Opcodes::Opcodes::None,
            params: Vec::new()
        }
    }
}
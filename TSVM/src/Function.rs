use Instruction;

#[derive(Clone, RustcDecodable)]
pub struct Function {
    pub name: String,
    pub inst: Vec<Instruction::Instruction>
}

impl Function {
    pub fn new() -> Function {
        Function {
            name: String::new(),
            inst: Vec::new()
        }
    }
}
use Instruction;

#[derive(RustcEncodable)]
pub struct Function {
    pub name: String,
    pub inst: Vec<Instruction::Instruction>
}

impl Function {
    pub fn new(name: &String, inst: &Vec<Instruction::Instruction>) -> Function {
        let n = name.clone();
        let i = inst.clone();
        Function {
            name: n,
            inst: i
        }
    }
}
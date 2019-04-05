use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::io::BufReader;
use Utilities;
use Function;
use InstructionCompiler;
use Instruction;

pub fn compile_funcs(filepath: String) -> Vec<Function::Function> {

    // Parse info
    let mut funcs: HashMap<String, Vec<String>> = HashMap::new();
    let mut out: Vec<Function::Function> = Vec::new();
    let mut insts: Vec<Instruction::Instruction> = Vec::new();
    let mut nm: String = String::new();

    // Read
    let mut in_func: bool = false;
    let mut name: String = String::new();
    let mut lns: Vec<String> = Vec::new();

    // File
    let file = File::open(filepath).unwrap();
    let mut buf = BufReader::new(file);
    let mut content: String = String::new();
    buf.read_to_string(&mut content).unwrap();
    let strs = content.split("\n").map(|s| s.to_string());

    for ln in strs {
        if !Utilities::is_whitespace(Utilities::copy(&ln)) {
            let tk = ln.split_whitespace().collect::<Vec<&str>>();
            let line = Utilities::copy(&ln);
            match tk[0] {
                "end" => if tk[1] == "func" {
                    in_func = false;
                    funcs.insert(name.to_string(), Utilities::copy_vec(&lns));
                    name.clear();
                    lns.clear();
                },
                "func" => {
                    name = tk[1].to_string();
                    in_func = true;
                },
                _ => {
                    if in_func {
                        lns.push(line);
                    }
                }
            }
        }
    }

    for (name, content) in funcs {
        nm = name;
        for ln in content {
            insts.push(InstructionCompiler::compile_instruction(&ln));
        }
        out.push(Function::Function::new(&nm, &insts));
        insts.clear();
    }

    out
}
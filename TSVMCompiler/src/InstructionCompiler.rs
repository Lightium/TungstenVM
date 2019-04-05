use Instruction;
use Opcodes;
use Utilities;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref USED_NAMES: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref NAME_AND_TYPE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref USED_POS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn compile_instruction(line: &str) -> Instruction::Instruction {
    if !Utilities::is_whitespace(Utilities::copy(&line.to_string())) {
        let tk = line.split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect::<Vec<String>>();
        match &tk[0][..] {
            "none" => {

                // If tk's length isn't 1 (tk[0] = "none"), panic.
                if tk.len() != 1 {
                    panic!("Invalid none operation");
                }

                Instruction::Instruction::new(Opcodes::Opcodes::None, Vec::new())
            },

            "var" => {

                // If tk's length isn't 5 (tk[0] = "var", tk[1] = "Name", etc.), panic.
                if tk.len() < 5 {
                    panic!("Invalid var operation");
                }

                let mut value: String = String::new();

                // Check if the name of the variable was already used
                if USED_NAMES.lock().unwrap().contains(&tk[1]) {
                    panic!("Variable {:?} already exists!", &tk[1]);
                }

                // Check value Type
                if !(&tk[2][..] == "StrType" || &tk[2][..] == "IntType" || &tk[2][..] == "DecType") {
                    panic!("Invalid value type: {:?}", &tk[2]);
                }

                // Check assignment symbol
                if tk[3] != "=" { panic!("Invalid assignment operator: {:?}", &tk[3]) }

                // If quotes are found in `line` (Variable), get the text between them
                if line.contains("\"") {
                    // Check for type mismatch
                    if &tk[2] != "StrType" { panic!("Type mismatch") }
                    let rgx = Regex::new("\"([^\"]*)\"").unwrap();
                    value = Utilities::rem_quotes(rgx.find(&line).unwrap().as_str().to_string());
                } else {
                    // Check for type mismatch
                    if &tk[2] == "StrType" { panic!("Type mismatch") }
                    if tk[4].parse::<i32>().is_err() && &tk[2] == "IntType" { panic!("Type mismatch") }
                    if tk[4].parse::<f32>().is_err() && &tk[2] == "DecType" { panic!("Type mismatch") }
                    value = tk[4].to_string()
                }

                // Store name and type
                NAME_AND_TYPE.lock().unwrap().insert(Utilities::copy_str(&tk[1]), Utilities::copy_str(&tk[2]));

                // Mark name as used
                USED_NAMES.lock().unwrap().push(Utilities::copy_str(&tk[1]));

                // Return
                Instruction::Instruction::new(Opcodes::Opcodes::Var, vec![Utilities::copy_str(&tk[1]),
                                                                          Utilities::copy_str(&tk[2]), value])
            }

            "set" => {
                // set var type = var1
                // If tk's length isn't 5 (tk[1] = "set", tk[1] = "Name", etc.), panic.
                if tk.len() < 5 {
                    panic!("Invalid set operation");
                }

                let mut value: String = String::new();

                if !USED_NAMES.lock().unwrap().contains(&tk[1]) {
                    panic!("Variable {:?} does not exist", &tk[1]);
                }

                if &tk[3][..] != "=" && &tk[3][..] != "+=" && &tk[3][..] != "-=" && &tk[3][..] != "/=" &&
                    &tk[3][..] != "*=" && &tk[3][..] != "%=" && &tk[3][..] != "!" {
                    panic!("Invalid assignment operator: {:?}", tk[3])
                }

                // Return
                Instruction::Instruction::new(Opcodes::Opcodes::Set, vec![Utilities::copy_str(&tk[1]), Utilities::copy_str(&tk[2]),
                                                                          Utilities::copy_str(&tk[3]), Utilities::copy_str(&tk[4])])
            }

            "print" => {

                // If tk's length isn't 2 (tk[0] = "print", tk[1] = "VariableName", etc.), panic.
                if tk.len() != 2 {
                    panic!("Invalid print operation");
                }

                // Check if variable exists
                if !USED_NAMES.lock().unwrap().contains(&tk[1]) {
                    panic!("Variable {:?} doesn't exist", &tk[1]);
                }

                Instruction::Instruction::new(Opcodes::Opcodes::Print, vec![Utilities::copy_str(&tk[1])])
            },

            "eq" => {

                // If tk's length isn't 3 (tk[0] = "eq", tk[1] = "VariableName", tk[2] = "VariableName2"), panic.
                if tk.len() != 3 {
                    panic!("Invalid none operation");
                }

                // Check if variable No. 1 exists
                if !USED_NAMES.lock().unwrap().contains(&tk[1]) {
                    panic!("Variable {:?} doesn't exist", &tk[1]);
                }

                // Check if variable No. 2 exists
                if !USED_NAMES.lock().unwrap().contains(&tk[2]) {
                    panic!("Variable {:?} doesn't exist", &tk[2]);
                }

                // Check type
                if NAME_AND_TYPE.lock().unwrap().get(&tk[1]) !=
                   NAME_AND_TYPE.lock().unwrap().get(&tk[2]) {
                    panic!("Type mismatch");
                }

                Instruction::Instruction::new(Opcodes::Opcodes::Equals, vec![Utilities::copy_str(&tk[1]),
                                                                             Utilities::copy_str(&tk[2])])
            },

            "lss" => {

                // If tk's length isn't 3 (tk[0] = "lss", tk[1] = "VariableName", tk[2] = "VariableName2"), panic.
                if tk.len() != 3 {
                    panic!("Invalid lss operation");
                }

                // Check if variable No. 1 exists
                if !USED_NAMES.lock().unwrap().contains(&tk[1].to_string()) {
                    panic!("Variable {:?} doesn't exist", &tk[1]);
                }

                // Check if variable No. 2 exists
                if !USED_NAMES.lock().unwrap().contains(&tk[2].to_string()) {
                    panic!("Variable {:?} doesn't exist", &tk[2]);
                }

                // Check type
                if !(NAME_AND_TYPE.lock().unwrap().get(&tk[1]) != Some(&"IntType".to_string()) ||
                    NAME_AND_TYPE.lock().unwrap().get(&tk[1]) != Some(&"DecType".to_string())) {
                    panic!("Incompatible type");
                }

                // Check type
                if !(NAME_AND_TYPE.lock().unwrap().get(&tk[2]) != Some(&"IntType".to_string()) ||
                    NAME_AND_TYPE.lock().unwrap().get(&tk[2]) != Some(&"DecType".to_string())) {
                    panic!("Incompatible type");
                }

                Instruction::Instruction::new(Opcodes::Opcodes::LessThan, vec![Utilities::copy_str(&tk[1]),
                                                                               Utilities::copy_str(&tk[2])])
            },

            "gtr" => {

                // If tk's length isn't 3 (tk[0] = "gtr", tk[1] = "VariableName", tk[2] = "VariableName2"), panic.
                if tk.len() != 3 {
                    panic!("Invalid gtr operation");
                }

                // Check if variable No. 1 exists
                if !USED_NAMES.lock().unwrap().contains(&tk[1]) {
                    panic!("Variable {:?} doesn't exist", &tk[1]);
                }

                // Check if variable No. 2 exists
                if !USED_NAMES.lock().unwrap().contains(&tk[2]) {
                    panic!("Variable {:?} doesn't exist", &tk[2]);
                }

                // Check type
                if !(NAME_AND_TYPE.lock().unwrap().get(&tk[1]) != Some(&"IntType".to_string()) ||
                    NAME_AND_TYPE.lock().unwrap().get(&tk[1]) != Some(&"DecType".to_string())) {
                    panic!("Incompatible type");
                }

                // Check type
                if !(NAME_AND_TYPE.lock().unwrap().get(&tk[2]) != Some(&"IntType".to_string()) ||
                    NAME_AND_TYPE.lock().unwrap().get(&tk[2]) != Some(&"DecType".to_string())) {
                    panic!("Incompatible type");
                }

                Instruction::Instruction::new(Opcodes::Opcodes::GreaterThan, vec![Utilities::copy_str(&tk[1]),
                                                                                  Utilities::copy_str(&tk[2])])
            },

            "or" => {

                // If tk's length isn't 3 (tk[0] = "or", tk[1] = "VariableName", tk[2] = "VariableName2"), panic.
                if tk.len() != 3 {
                    panic!("Invalid or operation");
                }

                // Check if variable No. 1 exists
                if !USED_NAMES.lock().unwrap().contains(&tk[1]) {
                    panic!("Variable {:?} doesn't exist", tk[1]);
                }

                // Check if variable No. 2 exists
                if !USED_NAMES.lock().unwrap().contains(&tk[2]) {
                    panic!("Variable {:?} doesn't exist", tk[2]);
                }

                // Check type
                if NAME_AND_TYPE.lock().unwrap().get(&tk[1]) != Some(&"IntType".to_string()) {
                    panic!("Incompatible type");
                }

                // Check type
                if NAME_AND_TYPE.lock().unwrap().get(&tk[2]) != Some(&"IntType".to_string()) {
                    panic!("Incompatible type");
                }

                Instruction::Instruction::new(Opcodes::Opcodes::Or, vec![Utilities::copy_str(&tk[1]), Utilities::copy_str(&tk[2])])
            },

            "and" => {
                // If tk's length isn't 3 (tk[0] = "and", tk[1] = "VariableName", tk[2] = "VariableName2"), panic.
                if tk.len() != 3 {
                    panic!("Invalid or operation");
                }

                // Check if variable exists
                if !USED_NAMES.lock().unwrap().contains(&tk[1]) {
                    panic!("Variable {:?} doesn't exist", &tk[1]);
                }

                // Check if variable exists
                if !USED_NAMES.lock().unwrap().contains(&tk[2]) {
                    panic!("Variable {:?} doesn't exist", &tk[2]);
                }

                // Check type
                if NAME_AND_TYPE.lock().unwrap().get(&tk[1]) != Some(&"IntType".to_string()) {
                    panic!("Incompatible type");
                }

                // Check type
                if NAME_AND_TYPE.lock().unwrap().get(&tk[2]) != Some(&"IntType".to_string()) {
                    panic!("Incompatible type");
                }

                Instruction::Instruction::new(Opcodes::Opcodes::And, vec![Utilities::copy_str(&tk[1]), Utilities::copy_str(&tk[2])])
            },

            "res" => {

                // Check if variable exists
                if !USED_NAMES.lock().unwrap().contains(&tk[1]) {
                    panic!("Variable {:?} doesn't exist", &tk[1]);
                }

                Instruction::Instruction::new(Opcodes::Opcodes::Res, vec![Utilities::copy_str(&tk[1])])
            },

            // Jump to position
            // ex.:
            // func SomeFunc
            // pos somepos
            // jmp somepos
            // func end
            "jmp" => Instruction::Instruction::new(Opcodes::Opcodes::Jump, vec![Utilities::copy_str(&tk[1])]),

            // does the same as jmp but only is res is true
            "jt" => Instruction::Instruction::new(Opcodes::Opcodes::JumpIfResTrue,
                                                  vec![tk[1].clone()]),

            "call" => Instruction::Instruction::new(Opcodes::Opcodes::Call,
                                                    vec![tk[1].clone()]),

            "callext" => {
                unimplemented!();
                //if tk.len() != 4 { panic!("Invalid callext operation") }
                //Instruction::Instruction::new(Opcodes::Opcodes::CallExt, vec![Utilities::copy(&tk[1]).to_string(),
                //                                                              Utilities::copy(&tk[2]).to_string(), Utilities::copy(&tk[3]).to_string()])
            },

            "pos" => {
                if USED_POS.lock().unwrap().contains(&tk[1]) {
                    panic!("Position {:?} is already exists", &tk[1]);
                }
                Instruction::Instruction::new(Opcodes::Opcodes::Pos, vec![tk[1].clone()])
            }

            x => panic!("Unrecognized operator: {:?}", x)
        }
    } else { Instruction::Instruction::new(Opcodes::Opcodes::None, Vec::new()) }
}

use Function;
use Opcodes;
use Instruction;
use libloading;
use std::collections::HashMap;

pub fn copy_vec<T: Clone>(vec: &Vec<T>) -> Vec<T> {
    vec.clone()
}

pub fn copy<T: Clone>(obj: &T) -> T {
    obj.clone()
}

pub fn get_func(funcs: Vec<Function::Function>, name: String) -> Function::Function {
    let mut out = Function::Function::new();
    for func in funcs { if func.name == name { out = func }}
    out
}

pub fn set_var<T: Clone + PartialEq>(vars: &mut Vec<Vec<T>>, name: T, value: T) -> Vec<Vec<T>> {
    let mut v: Vec<Vec<T>> = Vec::new();
    let cloned = vars.clone();
    let val = value.clone();
    for var in cloned {
        if var[0] == name {
            let _type = var[1].clone();
            let name = var[0].clone();
            let val = value.clone();
            v.push(vec![name, _type, val])
        } else {
            let var_clone = var.clone();
            v.push(var_clone);
        }
    }
    v
}

pub fn get_var(vars: Vec<Vec<String>>, name: String) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for var in vars { if var[0] == name { out = var }}
    out
}

fn reverse_of(bool_value: String) -> String {
    let owned = bool_value.to_owned();
    let s = &owned[..];
    match s {
        "true" => "false".to_string(),
        "false" => "true".to_string(),
        _ => panic!()
    }
}

pub fn run_fn(name: String, mut res: bool /* ! res is a register ! */, funcs: Vec<Function::Function>) {
    let mut vars = Vec::new();
    let mut index = 0;

    let func = get_func(copy(&funcs), name);

    while index < func.inst.len() {
        let i: Instruction::Instruction = copy(&func.inst[index]);
        match i.opcode {

            Opcodes::Opcodes::None => {},

            Opcodes::Opcodes::Var => {
                vars.push(vec![
                    copy(&i.params[0]), copy(&i.params[1]), copy(&i.params[2])]);
            },

            Opcodes::Opcodes::Set => {
                let owned = i.params[1].to_owned();
                let s = &owned[..];

                match s {
                    "StrType" => if i.params[2] == "=" {
                        let val = &get_var(copy_vec(&vars), copy(&i.params[3]))[2];
                        vars = set_var(&mut copy_vec(&vars),
                        copy(&i.params[0]), val.clone())
                    },

                    "IntType" => {
                        let owned = i.params[2].to_owned();
                        let s = &owned[..];

                        // set [[[varName]]] IntType = Value
                        let first: i32 =
                            get_var(copy_vec(&vars), copy(&i.params[0]))[2].parse::<i32>().expect(
                            "ValueError");

                        // set varName IntType = [[[Value]]]
                        let second: i32 =
                            get_var(copy_vec(&vars), copy(&i.params[3]))[2].parse::<i32>().expect(
                            "ValueError");

                        let mut result = 0;

                        // Self-Explanatory
                        match s {
                            "=" => result = second,

                            "-=" => result = first - second,

                            "+=" => result = first + second,

                            "*=" => result = first * second,

                            "/=" => result = first / second,

                            "%=" => result = first % second,

                            x => panic!("Unrecognized operator: {:?}. Did you modify the binary file?", x),
                        }

                        vars = set_var(&mut copy_vec(&vars), copy(&i.params[0]),
                        result.to_string());
                    },

                    "DecType" => {
                        let owned = i.params[2].to_owned();
                        let s = &owned[..];

                        // set [[[varName]]] DecType = Value
                        let first: f32 =
                            get_var(copy_vec(&vars), copy(&i.params[0]))[0].parse::<f32>().expect(
                                "ValueError");

                        // set varName DecType = [[[Value]]]
                        let second: f32 =
                            get_var(copy_vec(&vars), copy(&i.params[3]))[2].parse::<f32>().expect(
                            "ValueError");

                        // Self-Explanatory
                        let mut result = 0.0;

                        // Self-Explanatory
                        match s {
                            "=" => result = second,

                            "-=" => result = first - second,

                            "+=" => result = first + second,

                            "*=" => result = first * second,

                            "/=" => result = first / second,

                            "%=" => result = first % second,

                            x => panic!("Unrecognized operator: {:?}. Did you modify the binary file?", x),
                        }

                        vars = set_var(&mut copy_vec(&vars), copy(&i.params[0]),
                                       result.to_string())
                    },

                    x => panic!("Unrecognized operator: {:?}", x)
                }
            },

            Opcodes::Opcodes::Print => {
                let var = get_var(copy_vec(&vars), copy(&i.params[0]));
                println!("{:?}", var[2]);
            },

            Opcodes::Opcodes::Equals => {
                let var_1 = get_var(copy_vec(&vars), copy(&i.params[0]));
                let var_2 = get_var(copy_vec(&vars), copy(&i.params[1]));

                res = var_1[2] == var_2[2];
            },

            Opcodes::Opcodes::LessThan => {
                let var_1 = get_var(copy_vec(&vars), copy(&i.params[0]));
                let var_2 = get_var(copy_vec(&vars), copy(&i.params[1]));

                let parsed_1 = var_1[2].parse::<f32>().expect(
                    &format!("ERROR: Variable {} is not a valid number!", var_1[0]));

                let parsed_2 = var_2[2].parse::<f32>().expect(
                    &format!("ERROR: Variable {} is not a valid number!", var_2[0]));

                res = parsed_1 < parsed_2;
            },

            Opcodes::Opcodes::GreaterThan => {
                let var_1 = get_var(copy_vec(&vars), copy(&i.params[0]));
                let var_2 = get_var(copy_vec(&vars), copy(&i.params[1]));

                let parsed_1 = var_1[2].parse::<f32>().expect(
                    &format!("ERROR: Variable {} is not a valid number!", var_1[0]));

                let parsed_2 = var_2[2].parse::<f32>().expect(
                    &format!("ERROR: Variable {} is not a valid number!", var_2[0]));

                res = parsed_1 > parsed_2;
            },

            Opcodes::Opcodes::Or => {
                let var_1 = get_var(copy_vec(&vars), copy(&i.params[0]));
                let var_2 = get_var(copy_vec(&vars), copy(&i.params[1]));

                let parsed_1 = var_1[2].parse::<i32>().expect(
                    &format!("ERROR: Variable {} is not a valid number!", var_1[0]));

                let parsed_2 = var_2[2].parse::<i32>().expect(
                    &format!("ERROR: Variable {} is not a valid number!", var_2[0]));

                let mut bool_1 = if parsed_1 == 0 { false } else { true };
                let mut bool_2 = if parsed_2 == 0 { false } else { true };

                res = bool_1 | bool_2;
            },

            Opcodes::Opcodes::And => {
                let var_1 = get_var(copy_vec(&vars), copy(&i.params[0]));
                let var_2 = get_var(copy_vec(&vars), copy(&i.params[1]));

                let parsed_1 = var_1[2].parse::<i32>().expect(
                    &format!("ERROR: Variable {} is not a valid number!", var_1[0]));

                let parsed_2 = var_2[2].parse::<i32>().expect(
                    &format!("ERROR: Variable {} is not a valid number!", var_2[0]));

                let mut bool_1 = if parsed_1 == 0 { false } else { true };
                let mut bool_2 = if parsed_2 == 0 { false } else { true };

                res = bool_1 & bool_2;
            },

            Opcodes::Opcodes::Res => {
                set_var(&mut copy_vec(&vars), copy(&i.params[0]),
                        (res as i32).to_string());
            },

            Opcodes::Opcodes::Jump => {
                let idx = *(&i.params[0].parse::<usize>().expect("jmp ERROR: Invalid number"));
                index = idx;
            },

            Opcodes::Opcodes::JumpIfResTrue => {
                if res {
                    let idx = *(&i.params[0].parse::<usize>().expect("jmp ERROR: Invalid number"));
                    index = idx;
                }
            },

            Opcodes::Opcodes::Call => {
                run_fn(copy(&i.params[0]), res, copy(&funcs));
            },

            Opcodes::Opcodes::CallExt => {
                unimplemented!();
                //let res = call_func::<String>(copy(&i.params[1]), copy(&i.params[2]));
                //set_var(&mut copy_vec(&vars), copy(&i.params[0]), res);
            },

            Opcodes::Opcodes::Pos => {

            },

            _ => unimplemented!()
        }

        index += 1;
    }
}
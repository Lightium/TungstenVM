#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

mod Opcodes;
mod FunctionCompiler;
mod Utilities;
mod Function;
mod Instruction;
mod InstructionCompiler;

extern crate bincode;
extern crate rustc_serialize;
extern crate regex;
#[macro_use] extern crate lazy_static;

use std::env;
use std::fs::File;

fn main() {
    let args: Vec<_> = env::args().collect();
    let compiled: Vec<Function::Function> = FunctionCompiler::compile_funcs(args[1].to_string());
    let mut file = File::create(&args[2]).expect("Unable to create file");
    bincode::rustc_serialize::encode_into(&compiled, &mut file, bincode::SizeLimit::Infinite).expect(
    "Unable to write to file");
    println!("SUCCESS: Successfully compiled!");
}

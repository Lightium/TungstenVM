extern crate bincode;
extern crate rustc_serialize;
extern crate libloading;

mod Function;
mod Instruction;
mod Opcodes;
mod VM;

use std::env;
use std::fs::File;

fn main() {
    // Registers
    let res = false;

    // Get arguments
    let args: Vec<_> = env::args().collect();

    // Read binary file
    let mut f = File::open(&args[1]).expect("Unable to open file");
    let funcs: Vec<Function::Function> = bincode::rustc_serialize::decode_from(&mut f, bincode::SizeLimit::Infinite).expect(
    "Failed to read bytes");

    VM::run_fn("Main".to_string(), res, VM::copy(&funcs));
}

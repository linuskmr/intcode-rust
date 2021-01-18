mod mode;
mod opcode;
mod program;

use std::{fs, env};
use crate::program::Program;

fn main() {
    // Read filename from command arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Read file
    let mut content = fs::read_to_string(filename).expect("Could not read file");

    // Beautify content
    content = content.replace("\n", "");
    let ints : Vec<i64> = content.split(",").map(|num| num.parse().unwrap()).collect();
    drop(content);

    // Create new program and execute it
    let mut p = Program::new(ints);
    p.exec();
}

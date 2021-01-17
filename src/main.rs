mod mode;
mod opcode;
mod program;

use std::{fs, env};
use crate::program::Program;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Filename {}", filename);
    let mut content = fs::read_to_string(filename).expect("Could not read file");
    content = content.replace("\n", "");
    let ints : Vec<i64> = content.split(",").map(|num| num.parse().unwrap()).collect();
    drop(content);
    let mut p = Program::new(ints);
    p.exec();
}

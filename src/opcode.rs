use crate::program::Program;
use std::io;
use std::io::Write;
use core::fmt;

pub struct Opcode {
    name: &'static str,
    pub function: fn(&mut Program),
    pub num_of_params: usize,
}

const OPCODES: [Opcode; 10] = [
    Opcode{name: "no op", function: no_op, num_of_params: 0},
    Opcode{name: "add", function: add, num_of_params: 3},
    Opcode{name: "multiply", function: multiply, num_of_params: 3},
    Opcode{name: "input", function: input, num_of_params: 1},
    Opcode{name: "output", function: output, num_of_params: 1},
    Opcode{name: "jump non-zero", function: jump_non_zero, num_of_params: 2},
    Opcode{name: "jump zero", function: jump_zero, num_of_params: 2},
    Opcode{name: "less than", function: less_than, num_of_params: 3},
    Opcode{name: "equal", function: equal, num_of_params: 3},
    Opcode{name: "add to relative base", function: add_relative_base, num_of_params: 1},
];

impl Opcode {
    pub fn new(num: i64) -> &'static Opcode {
        let opcode_number = (num % 100) as usize;
        OPCODES.get(opcode_number).expect(&*format! {"Invalid opcode {}", opcode_number})
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn no_op(_: &mut Program) {}

/// Adds to numbers together and stores the result in the third parameter.
fn add(p: &mut Program) {
    p.code[p.arg_indices[2]] = p.code[p.arg_indices[0]] + p.code[p.arg_indices[1]]
}

/// Multiplies to numbers together and stores the result in the third parameter.
fn multiply(p: &mut Program) {
    p.code[p.arg_indices[2]] = p.code[p.arg_indices[0]] * p.code[p.arg_indices[1]]
}

/// Inputs a number and stores the result in the second parameter.
fn input(p: &mut Program) {
    // Write input prompt
    print!("Input: ");
    io::stdout().flush().unwrap();

    // Read line froms stdin
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

    // Parse line to number
    let trimmed_input = input_text.trim();
    p.code[p.arg_indices[0]] = trimmed_input.parse().expect("failed to parse integer");
}

/// Outputs the number of the first parameter.
fn output(p: &mut Program) {
    println!("Output: {}", p.code[p.arg_indices[0]]);
}

/// Jumps to the position of the second parameter if the first parameter is non-zero.
fn jump_non_zero(p: &mut Program) {
    if p.code[p.arg_indices[0]] != 0 {
        p.ip = p.code[p.arg_indices[1]] as usize;
        p.move_ip = false;
    }
}

/// Jumps to the position of the second parameter if the first parameter is zero.
fn jump_zero(p: &mut Program) {
    if p.code[p.arg_indices[0]] == 0 {
        p.ip = p.code[p.arg_indices[1]] as usize;
        p.move_ip = false;
    }
}

/// Writes 1 to the third parameter if the first parameter is less than the second parameter.
/// Otherwise writes 0.
fn less_than(p: &mut Program) {
    p.code[p.arg_indices[2]] = bool_to_int(p.code[p.arg_indices[0]] < p.code[p.arg_indices[1]]);
}

/// Writes 1 to the third parameter if the first parameter is equal to the second parameter.
/// Otherwise writes 0.
fn equal(p: &mut Program) {
    p.code[p.arg_indices[2]] = bool_to_int(p.code[p.arg_indices[0]] == p.code[p.arg_indices[1]]);
}

fn add_relative_base(p: &mut Program) {
    p.rel_base += p.code[p.arg_indices[0]];
}

#[allow(dead_code)]
fn end(p: &mut Program) {
    p.finish = true;
}

fn bool_to_int(b: bool) -> i64 {
    if b {
        1
    } else {
        0
    }
}

#[allow(dead_code)]
fn int_to_bool(i: i64) -> bool {
    if i == 0 {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut program = Program{
            code: vec![1, 2, 0],
            ip: 0,
            move_ip: true,
            rel_base: 0,
            arg_indices: vec![0, 1, 2],
            finish: false
        };
        // Positive add
        add(&mut program);
        assert_eq!(program.code, vec![1, 2, 3]);

        // Partial negative add
        program.code = vec![-3, 2, 0];
        add(&mut program);
        assert_eq!(program.code, vec![-3, 2, -1]);

        // Negative add
        program.code = vec![-3, -2, 0];
        add(&mut program);
        assert_eq!(program.code, vec![-3, -2, -5]);
    }

    #[test]
    fn test_multiply() {
        let mut program = Program{
            code: vec![4, 2, 0],
            ip: 0,
            move_ip: true,
            rel_base: 0,
            arg_indices: vec![0, 1, 2],
            finish: false
        };
        // Positive multiply
        multiply(&mut program);
        assert_eq!(program.code, vec![4, 2, 8]);

        // Negative multiply
        program.code = vec![-3, 2, 0];
        multiply(&mut program);
        assert_eq!(program.code, vec![-3, 2, -6]);

        // Negative * Negative = Positive
        program.code = vec![-3, -2, 0];
        multiply(&mut program);
        assert_eq!(program.code, vec![-3, -2, 6]);
    }

    #[test]
    fn test_jump_non_zero() {
        let mut program = Program{
            code: vec![1, 0, 42],
            ip: 0,
            move_ip: true,
            rel_base: 0,
            arg_indices: vec![1, 2],
            finish: false
        };
        // Don't jump
        jump_non_zero(&mut program);
        assert_eq!(program.ip, 0);
        assert_eq!(program.move_ip, true);

        // Jump
        program.code[1] = 3;
        jump_non_zero(&mut program);
        assert_eq!(program.ip, 42);
        assert_eq!(program.move_ip, false);
    }

    #[test]
    fn test_jump_zero() {
        let mut program = Program{
            code: vec![1, 3, 42],
            ip: 0,
            move_ip: true,
            rel_base: 0,
            arg_indices: vec![1, 2],
            finish: false
        };
        // Don't jump
        jump_zero(&mut program);
        assert_eq!(program.ip, 0);
        assert_eq!(program.move_ip, true);

        // Jump
        program.code[1] = 0;
        jump_zero(&mut program);
        assert_eq!(program.ip, 42);
        assert_eq!(program.move_ip, false);
    }

    #[test]
    fn test_add_relative_base() {
        let mut program = Program{
            code: vec![42],
            ip: 0,
            move_ip: true,
            rel_base: 0,
            arg_indices: vec![0],
            finish: false
        };
        // Positive add to relative base
        add_relative_base(&mut program);
        assert_eq!(program.rel_base, 42);

        // Negative add to relative base
        program.code = vec![-100];
        add_relative_base(&mut program);
        assert_eq!(program.rel_base, -58);
    }
}
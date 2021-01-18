use crate::program::Program;
use std::io;
use std::io::Write;
use core::fmt;

pub struct Instruction {
    /// The name of the instruction.
    name: &'static str,
    /// The function computing this instruction.
    pub compute_fn: fn(&mut Program),
    /// The number of parameters required by this instruction.
    pub num_of_params: usize,
}

const OPCODES: [Instruction; 10] = [
    Instruction {name: "no op", compute_fn: no_op, num_of_params: 0},
    Instruction {name: "add", compute_fn: add, num_of_params: 3},
    Instruction {name: "multiply", compute_fn: multiply, num_of_params: 3},
    Instruction {name: "input", compute_fn: input, num_of_params: 1},
    Instruction {name: "output", compute_fn: output, num_of_params: 1},
    Instruction {name: "jump non-zero", compute_fn: jump_non_zero, num_of_params: 2},
    Instruction {name: "jump zero", compute_fn: jump_zero, num_of_params: 2},
    Instruction {name: "less than", compute_fn: less_than, num_of_params: 3},
    Instruction {name: "equal", compute_fn: equal, num_of_params: 3},
    Instruction {name: "add to relative base", compute_fn: add_relative_base, num_of_params: 1},
];

impl Instruction {
    pub fn new(opcode: i64) -> &'static Instruction {
        let opcode_number = (opcode % 100) as usize;
        OPCODES.get(opcode_number).expect(&*format! {"Invalid opcode {}", opcode_number})
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn no_op(_: &mut Program) {}

/// Adds to numbers together and stores the result in the third parameter.
fn add(p: &mut Program) {
    p.code[p.param_indices[2]] = p.code[p.param_indices[0]] + p.code[p.param_indices[1]]
}

/// Multiplies to numbers together and stores the result in the third parameter.
fn multiply(p: &mut Program) {
    p.code[p.param_indices[2]] = p.code[p.param_indices[0]] * p.code[p.param_indices[1]]
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
    p.code[p.param_indices[0]] = trimmed_input.parse().expect("failed to parse integer");
}

/// Outputs the number of the first parameter.
fn output(p: &mut Program) {
    println!("Output: {}", p.code[p.param_indices[0]]);
}

/// Jumps to the position of the second parameter if the first parameter is non-zero.
fn jump_non_zero(p: &mut Program) {
    if p.code[p.param_indices[0]] != 0 {
        p.ip = p.code[p.param_indices[1]] as usize;
        p.move_ip = false;
    }
}

/// Jumps to the position of the second parameter if the first parameter is zero.
fn jump_zero(p: &mut Program) {
    if p.code[p.param_indices[0]] == 0 {
        p.ip = p.code[p.param_indices[1]] as usize;
        p.move_ip = false;
    }
}

/// Writes 1 to the third parameter if the first parameter is less than the second parameter.
/// Otherwise writes 0.
fn less_than(p: &mut Program) {
    p.code[p.param_indices[2]] = bool_to_int(p.code[p.param_indices[0]] < p.code[p.param_indices[1]]);
}

/// Writes 1 to the third parameter if the first parameter is equal to the second parameter.
/// Otherwise writes 0.
fn equal(p: &mut Program) {
    p.code[p.param_indices[2]] = bool_to_int(p.code[p.param_indices[0]] == p.code[p.param_indices[1]]);
}

/// Adds the first parameter to the relative base register of the program.
fn add_relative_base(p: &mut Program) {
    p.rel_base += p.code[p.param_indices[0]];
}

/// Converts a bool to an integer. If b is true returns 1, otherwise 0.
fn bool_to_int(b: bool) -> i64 {
    if b {
        1
    } else {
        0
    }
}

/// Converts an integer into a bool. If i == 0 returns true, otherwise false.
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
            param_indices: vec![0, 1, 2],
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
            param_indices: vec![0, 1, 2],
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
            param_indices: vec![1, 2],
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
            param_indices: vec![1, 2],
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
            param_indices: vec![0],
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
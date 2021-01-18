use crate::program::Program;
use std::fmt;

// A mode of a parameter.
pub struct Mode {
    /// The name of this mode.
    name: &'static str,
    /// The function resolving the index of the parameter.
    pub index_resolving_fn: fn(&mut Program, usize) -> usize,
}

pub const MODES: [Mode; 3] = [
    Mode{name: "position mode", index_resolving_fn: position},
    Mode{name: "immediate mode", index_resolving_fn: immediate},
    Mode{name: "relative base mode", index_resolving_fn: relative_base}
];

impl fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Calculates the index of the parameter in position mode. Returns the value of the parameter at
/// the given index.
fn position(program: &mut Program, index: usize) -> usize {
    program.code[index] as usize
}

/// Calculates the index of the parameter in immediate mode. Returns the index.
fn immediate(_: &mut Program, index: usize) -> usize {
    index
}

/// Calculates the index of the parameter in relative base mode. Returns the index of the
/// parameter at the given index added with the relative base register of the program.
fn relative_base(program: &mut Program, index: usize) -> usize {
    (program.code[index] + program.rel_base) as usize
}

pub struct ModeList(pub Vec<&'static Mode>);

impl ModeList {
    pub fn new(modes: i64, num: usize) -> Self {
        // Remove the two-digit opcode
        let modes = modes / 100;
        // Allocate vec for the modes
        let mut mode_vec = vec![&MODES[0]; num];
        for i in 0..num {
            // Extract each node by division and modulo. The division results in position mode(0),
            // if no parameter is specified.
            let mode_number = ((modes / 10i64.pow(i as u32)) % 10) as usize;
            mode_vec[i] = MODES.get(mode_number).expect("Invalid mode");
        }
        Self(mode_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let mut program = Program{
            code: vec![42],
            ip: 0,
            move_ip: true,
            rel_base: 0,
            param_indices: vec![],
            finish: false
        };
        assert_eq!(42, position(&mut program, 0));
    }

    #[test]
    fn test_immediate() {
        let mut program = Program{
            code: vec![42],
            ip: 0,
            move_ip: true,
            rel_base: 0,
            param_indices: vec![],
            finish: false
        };
        assert_eq!(0, immediate(&mut program, 0));
    }

    #[test]
    fn test_relative_base() {
        let mut program = Program{
            code: vec![-10],
            ip: 0,
            move_ip: true,
            rel_base: 42,
            param_indices: vec![],
            finish: false
        };
        assert_eq!(32, relative_base(&mut program, 0));
    }
}
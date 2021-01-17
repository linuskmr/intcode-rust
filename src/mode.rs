use crate::program::Program;
use std::fmt;

pub struct Mode {
    name: &'static str,
    pub function: fn(&mut Program, usize) -> usize
}

/*impl fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mode").field("name", &self.name).finish()
    }
}*/

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

const MODES: [Mode; 3] = [
    // Position mode
    Mode{name: "position mode", function: position},

    // Immediate mode
    Mode{name: "immediate mode", function: immediate},

    // Relative base mode
    Mode{name: "relative base mode", function: relative_base}
];

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
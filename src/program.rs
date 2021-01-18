use crate::opcode::Instruction;
use crate::mode::ModeList;
use std::time::Instant;

pub struct Program {
    /// The source code of the program.
    pub code: Vec<i64>,
    /// The instruction pointer.
    pub ip: usize,
    /// Indicates whether the ip should be moved after the current instruction.
    pub move_ip: bool,
    /// The relative base register.
    pub rel_base: i64,
    /// The arguments for the current instruction.
    pub param_indices: Vec<usize>,
    /// Indicates whether the program is finished.
    pub finish: bool,
}

impl Program {
    pub fn new(code: Vec<i64>) -> Self {
        Program {
            code,
            ip: 0,
            move_ip: true,
            rel_base: 0,
            param_indices: vec![],
            finish: false,
        }
    }

    /// Executes this program
    pub fn exec(&mut self) {
        let start_time = Instant::now();
        while !self.finish {
            self.move_ip = true;
            // Parse opcode, modes and matching indices for each parameter.
            let opcode = self.code[self.ip];
            if opcode == 99 {
                // End program
                self.finish = true;
                continue;
            }
            let instruction = Instruction::new(opcode);
            let modes = ModeList::new(opcode, instruction.num_of_params);
            self.get_param_indices(modes);
            // println!("Executing opcode {:?}", opcode);
            // println!("Modes {:?}", modes.0);
            // println!("Params {:?}", self.arg_indices);

            // Execute function for instruction
            let instruction_function = instruction.compute_fn;
            instruction_function(self);

            // Move the instruction pointer if desired
            if self.move_ip {
                // Move instruction pointer by one for the opcode + the number of parameters
                self.ip += 1 + instruction.num_of_params;
            }
        }
        println!("Execution took {:?}", start_time.elapsed());
    }

    /// Processes the mode_list into a list of indices of the parameters.
    fn get_param_indices(&mut self, modes: ModeList) {
        // Allocate vec for the parameters.
        self.param_indices = vec![0; modes.0.len()];
        for (i, mode) in modes.0.iter().enumerate() {
            // The index of the parameter is located at the instruction pointer + one for the
            // opcode + the offset of parameters read before.
            let parameter_index = self.ip + 1 + i;
            // Here the function for the current mode is executed, which returns the index of the
            // parameter.
            let f = mode.index_resolving_fn;
            self.param_indices[i] = f(self, parameter_index);
        }
        self.grow_code_if_necessary()
    }

    /// Grows the code vector if a parameter index is out of range
    fn grow_code_if_necessary(&mut self) {
        let max = *self.param_indices.iter().max().unwrap_or(&0);
        if max >= self.code.len() {
            println!("Expanded code register from {} to {}", self.code.len(), max+42);
            self.code.resize(max+42, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mode::MODES;

    #[test]
    fn test_grow_code_if_necessary() {
        let mut program = Program{
            code: vec![1, 2, 3],
            ip: 0,
            move_ip: true,
            rel_base: 0,
            param_indices: vec![0, 42, 6],
            finish: false
        };
        // Try to access index 42, so grow vec to length >= 42+1
        program.grow_code_if_necessary();
        assert!(program.code.len() >= 42+1);
    }

    #[test]
    fn test_get_param_indices() {
        let mut program = Program{
            code: vec![42, 2, 0, 0],
            ip: 0,
            move_ip: true,
            rel_base: 1,
            param_indices: vec![],
            finish: false
        };
        // Starting with ip = 1, because ip points to the opcode
        // Value 2 in position mode should result in index 2
        // Value 0 in immediate mode should result in index 2
        // Value 0 in relative base mode should result in index 1
        let modes = ModeList(vec![&MODES[0], &MODES[1], &MODES[2]]);
        program.get_param_indices(modes);
        assert!(program.param_indices >= vec![2, 2, 1]);
    }
}
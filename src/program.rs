use crate::opcode::Opcode;
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
    pub arg_indices: Vec<usize>,
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
            arg_indices: vec![],
            finish: false
        }
    }

    /// Executes this program
    pub fn exec(&mut self) {
        let start_time = Instant::now();
        while !self.finish {
            self.move_ip = true;

            // Parse opcode, modes and matching indices for each parameter.
            let opcode_number = self.code[self.ip];
            if opcode_number == 99 {
                // End program
                self.finish = true;
                continue;
            }
            let opcode = Opcode::new(opcode_number);
            // println!("Executing opcode {:?}", opcode);
            let modes = ModeList::new(opcode_number, opcode.num_of_params);
            // println!("Modes {:?}", modes.0);
            self.get_arg_indices(modes);
            // println!("Params {:?}", self.arg_indices);

            // Execute function for opcode
            let f = opcode.function;
            f(self);

            // Move the instruction pointer if desired
            if self.move_ip {
                // Move instruction pointer by one for the opcode + the number of parameters
                self.ip += 1 + opcode.num_of_params;
            }
        }
        println!("Execution took {:?}", start_time.elapsed())
    }

    /// Processes the mode_list into a list of indices of the parameters.
    fn get_arg_indices(&mut self, modes: ModeList) {
        // Allocate vec for the parameters.
        self.arg_indices = vec![0; modes.0.len()];
        for (i, mode) in modes.0.iter().enumerate() {
            // The index of the parameter is located at the instruction pointer + one place
            // further back + the offset of parameters read before.
            let parameter_index = self.ip + 1 + i;
            // Here the function for the current mode is executed, which returns the index of the
            // parameter.
            let f = mode.function;
            self.arg_indices[i] = f(self, parameter_index);
        }
        self.grow_code_if_necessary()
    }

    /// Grows the code vector if a parameter index is out of range
    fn grow_code_if_necessary(&mut self) {
        let max = *self.arg_indices.iter().max().unwrap_or(&0);
        if max >= self.code.len() {
            println!("Grow code register from {} to {}", self.code.len(), max+1);
            self.code.resize(max+1 + 42, 0);
        }
    }
}




















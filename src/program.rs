use crate::opcode::Opcode;
use crate::mode::ModeList;

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
        let mut c = code;
        c.resize(4096, 0);
        Program {
            code: c,
            ip: 0,
            move_ip: true,
            rel_base: 0,
            arg_indices: vec![],
            finish: false
        }
    }

    /// Executes this program
    pub fn exec(&mut self) {
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
    }

    pub fn get(&self, index: usize) -> i64 {
        self.code[index]
    }

    pub fn set(&mut self, index: usize, value: i64) {
        self.code[index] = value
    }
}




















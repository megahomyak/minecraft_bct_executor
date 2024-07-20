use crate::{data_string, program_string};

pub struct BCT {
    data: data_string::DataString,
    program: program_string::ProgramString
}

impl BCT {
    pub fn new(data: data_string::DataString, program: program_string::ProgramString) -> Self {
        Self { data, program }
    }

    pub fn step(&mut self) {
        match self.program.next_cycling() {
            None => (),
            Some(b) => match b {
                false => self.data.delete_leftmost(),
                true => {
                    let x = unsafe { self.program.next_cycling().unwrap_unchecked() };
                    if Some(self.data.peek_leftmost()) == true {}
                }
            }
        }
    }

    pub fn run(&mut self) {}
}

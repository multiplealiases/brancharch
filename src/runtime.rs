use crate::machine::Machine;
use crate::opcode::OpCode;
pub trait Runtime {
    fn run(&mut self);
    fn run_random();
}

impl Runtime for Machine {
    fn run(&mut self) {
        match OpCode::from(self.fetch_inst()) {
            OpCode::Nop => self.advance(),
        }
    }
    fn run_random() {
        let mut machine = Machine::new_random();
        machine.run();
    }
}

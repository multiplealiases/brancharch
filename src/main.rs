mod machine;
mod opcode;
mod runtime;

use crate::machine::Machine;
use crate::runtime::Runtime;
fn main() {
    Machine::run_random()
}


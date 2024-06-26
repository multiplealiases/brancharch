use crate::machine::Machine;
use crate::opcode::OpCode;

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;
const REG_D: usize = 3;

pub trait Runtime {
    fn run(&mut self);
    fn step(&mut self);
    fn run_random();
}

impl Runtime for Machine {
    fn run(&mut self) {
        while !self.halt {
            #[cfg(feature = "debug-trace-execution")]
            print!("regs: {:?}, ip: {}, flag: {}, ", self.regs(), self.ip(), self.flag());
            self.step();
            #[cfg(feature = "debug-trace-execution")]
            println!("");
        }
    }
    fn step(&mut self) {
        let inst = self.fetch_inst();
        #[cfg(feature = "debug-trace-execution")]
        print!("inst: {:?} ", inst);
        match inst {
            OpCode::Nop => {},
            OpCode::LoadA => {
                let address = self.fetch_two();
                self.load(address, REG_A);
            }
            OpCode::LoadB => {
                let address = self.fetch_two();
                self.load(address, REG_B);
            }
            OpCode::LoadC => {
                let address = self.fetch_two();
                self.load(address, REG_C);
            }
            OpCode::LoadD => {
                let address = self.fetch_two();
                self.load(address, REG_D);
            }
            OpCode::StoreA => {
                let address = self.fetch_two();
                self.store(REG_A, address);
            }
            OpCode::StoreB => {
                let address = self.fetch_two();
                self.store(REG_B, address);
            }
            OpCode::StoreC => {
                let address = self.fetch_two();
                self.store(REG_C, address);
            }
            OpCode::StoreD => {
                let address = self.fetch_two();
                self.store(REG_D, address);
            }
            OpCode::SetFlag => {
                self.flag_set();
            }
            OpCode::ClearFlag => {
                self.flag_clear();
            }
            OpCode::AddAA => {
                self.add(REG_A, REG_A);
            }
            OpCode::AddBA => {
                self.add(REG_B, REG_A);
            }
            OpCode::AddCA => {
                self.add(REG_C, REG_A);
            }
            OpCode::AddDA => {
                self.add(REG_D, REG_A);
            }
            OpCode::AddBB => {
                self.add(REG_B, REG_B);
            }
            OpCode::AddBC => {
                self.add(REG_B, REG_C);
            }
            OpCode::AddBD => {
                self.add(REG_B, REG_D);
            }
            OpCode::AddCB => {
                self.add(REG_C, REG_B);
            }
            OpCode::AddCC => {
                self.add(REG_C, REG_C);
            }
            OpCode::AddCD => {
                self.add(REG_C, REG_D);
            }
            OpCode::AddDB => {
                self.add(REG_D, REG_B);
            }
            OpCode::AddDC => {
                self.add(REG_D, REG_C);
            }
            OpCode::AddDD => {
                self.add(REG_D, REG_D);
            }
            OpCode::SubAA => {
                self.sub(REG_A, REG_A);
            }
            OpCode::SubBA => {
                self.sub(REG_B, REG_A);
            }
            OpCode::SubCA => {
                self.sub(REG_C, REG_A);
            }
            OpCode::SubDA => {
                self.sub(REG_D, REG_A);
            }
            OpCode::SubBB => {
                self.sub(REG_B, REG_B);
            }
            OpCode::SubBC => {
                self.sub(REG_B, REG_C);
            }
            OpCode::SubBD => {
                self.sub(REG_B, REG_D);
            }
            OpCode::SubCB => {
                self.sub(REG_C, REG_B);
            }
            OpCode::SubCC => {
                self.sub(REG_C, REG_C);
            }
            OpCode::SubCD => {
                self.sub(REG_C, REG_D);
            }
            OpCode::SubDB => {
                self.sub(REG_D, REG_B);
            }
            OpCode::SubDC => {
                self.sub(REG_D, REG_C);
            }
            OpCode::SubDD => {
                self.sub(REG_D, REG_D);
            }
            OpCode::Jmp => {
                let address = self.fetch_two();
                self.jump(address);
            }
            OpCode::JmpRel => {
                let address = self.fetch_two();
                self.advance_by(address);
            }
            OpCode::WriteIp => {
                let address = self.fetch_two();
                let (lower, upper) = self.ip().to_le_bytes().into();
                self.poke_mem(address, lower);
                self.poke_mem(address + 1, upper);
            }
            OpCode::Branch => {
                let displacement = self.fetch_one();
                self.branch(displacement);
            }
            OpCode::LessThanA => {
                let address = self.fetch_two();
                self.less_than(REG_A, address)
            }
            OpCode::GreaterThanA => {
                let address = self.fetch_two();
                self.greater_than(REG_A, address)                
            }
            OpCode::EqualToA => {
                let address = self.fetch_two();
                self.equal_to(REG_A, address)
            }

            OpCode::LessThanB => {
                let address = self.fetch_two();
                self.less_than(REG_A, address)
            }
            OpCode::GreaterThanB => {
                let address = self.fetch_two();
                self.greater_than(REG_A, address)                
            }
            OpCode::EqualToB => {
                let address = self.fetch_two();
                self.equal_to(REG_A, address)                
            }
            
            OpCode::LessThanC => {
                let address = self.fetch_two();
                self.less_than(REG_A, address)                
            }
            OpCode::GreaterThanC => {
                let address = self.fetch_two();
                self.greater_than(REG_A, address)                
            }
            OpCode::EqualToC => {
                let address = self.fetch_two();
                self.equal_to(REG_A, address)                
            }

            OpCode::LessThanD => {
                let address = self.fetch_two();
                self.less_than(REG_D, address)                
            }
            OpCode::GreaterThanD => {
                let address = self.fetch_two();
                self.greater_than(REG_D, address)                
            }
            OpCode::EqualToD => {
                let address = self.fetch_two();
                self.equal_to(REG_D, address)                
            }

            OpCode::AndAA => {
                self.and(REG_A, REG_A);
            }
            OpCode::AndBA => {
                self.and(REG_B, REG_A);
            }
            OpCode::AndCA => {
                self.and(REG_C, REG_A);
            }
            OpCode::AndDA => {
                self.and(REG_D, REG_A);
            }
            OpCode::AndBB => {
                self.and(REG_B, REG_B);
            }
            OpCode::AndBC => {
                self.and(REG_B, REG_C);
            }
            OpCode::AndBD => {
                self.and(REG_B, REG_D);
            }
            OpCode::AndCB => {
                self.and(REG_C, REG_B);
            }
            OpCode::AndCC => {
                self.and(REG_C, REG_C);
            }
            OpCode::AndCD => {
                self.and(REG_C, REG_D);
            }
            OpCode::AndDB => {
                self.and(REG_D, REG_B);
            }
            OpCode::AndDC => {
                self.and(REG_D, REG_C);
            }
            OpCode::AndDD => {
                self.and(REG_D, REG_D);
            }

            OpCode::XorAA => {
                self.xor(REG_A, REG_A);
            }
            OpCode::XorBA => {
                self.xor(REG_B, REG_A);
            }
            OpCode::XorCA => {
                self.xor(REG_C, REG_A);
            }
            OpCode::XorDA => {
                self.xor(REG_D, REG_A);
            }
            OpCode::XorBB => {
                self.xor(REG_B, REG_B);
            }
            OpCode::XorBC => {
                self.xor(REG_B, REG_C);
            }
            OpCode::XorBD => {
                self.xor(REG_B, REG_D);
            }
            OpCode::XorCB => {
                self.xor(REG_C, REG_B);
            }
            OpCode::XorCC => {
                self.xor(REG_C, REG_C);
            }
            OpCode::XorCD => {
                self.xor(REG_C, REG_D);
            }
            OpCode::XorDB => {
                self.xor(REG_D, REG_B);
            }
            OpCode::XorDC => {
                self.xor(REG_D, REG_C);
            }
            OpCode::XorDD => {
                self.xor(REG_D, REG_D);
            }

            OpCode::OrAA => {
                self.or(REG_A, REG_A);
            }
            OpCode::OrBA => {
                self.or(REG_B, REG_A);
            }
            OpCode::OrCA => {
                self.or(REG_C, REG_A);
            }
            OpCode::OrDA => {
                self.or(REG_D, REG_A);
            }
            OpCode::OrBB => {
                self.or(REG_B, REG_B);
            }
            OpCode::OrBC => {
                self.or(REG_B, REG_C);
            }
            OpCode::OrBD => {
                self.or(REG_B, REG_D);
            }
            OpCode::OrCB => {
                self.or(REG_C, REG_B);
            }
            OpCode::OrCC => {
                self.or(REG_C, REG_C);
            }
            OpCode::OrCD => {
                self.or(REG_C, REG_D);
            }
            OpCode::OrDB => {
                self.or(REG_D, REG_B);
            }
            OpCode::OrDC => {
                self.or(REG_D, REG_C);
            }
            OpCode::OrDD => {
                self.or(REG_D, REG_D);
            }

            OpCode::RolA => {
                let shift = self.fetch_one();
                self.rotate_left(REG_A, shift)
            }
            OpCode::RolB => {
                let shift = self.fetch_one();
                self.rotate_left(REG_B, shift)
            }
            OpCode::RolC => {
                let shift = self.fetch_one();
                self.rotate_left(REG_C, shift)
            }
            OpCode::RolD => {
                let shift = self.fetch_one();
                self.rotate_left(REG_D, shift)
            }

            OpCode::RorA => {
                let shift = self.fetch_one();
                self.rotate_right(REG_A, shift)
            }
            OpCode::RorB => {
                let shift = self.fetch_one();
                self.rotate_right(REG_B, shift)
            }
            OpCode::RorC => {
                let shift = self.fetch_one();
                self.rotate_right(REG_C, shift)
            }
            OpCode::RorD => {
                let shift = self.fetch_one();
                self.rotate_right(REG_D, shift)
            }

            OpCode::AttemptHalt => {
                let a = self.peek_reg(REG_A) == 0xFF;
                let b = self.peek_reg(REG_B) == 0xFF;
                let c = self.peek_reg(REG_C) == 0xFF;
                let d = self.peek_reg(REG_D) == 0xFF;
                if a & b & c & d & self.flag() {
                    self.halt()
                }
            }
            OpCode::Print => {
                let address = self.fetch_two();
                self.print_byte(address);
            }

        }
    }
    fn run_random() {
        let mut machine = Machine::new_random();
        machine.run();
    }
}

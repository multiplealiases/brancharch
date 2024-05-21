use rand::prelude::*;
const MEMORY_SIZE: usize = 64 * (1 << 10);
pub struct Machine {
    regs: [u8; 4],
    ip: u16,
    memory: [u8; MEMORY_SIZE],
    flag: bool,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            regs: [0u8; 4],
            ip: 0,
            memory: [0u8; MEMORY_SIZE],
            flag: false,
        }
    }
    pub fn new_random() -> Machine {
        let mut rng = rand::thread_rng();
        let mut machine = Self::new();
        machine.memory.iter_mut().for_each(|i| *i = rng.gen());
        machine
    }
    pub fn fetch_inst(&mut self) -> u8 {
        let inst = self.memory[self.ip as usize];
        self.advance();
        inst
    }
    pub fn advance(&mut self) {
        self.ip = self.ip.wrapping_add(1);
    }
    pub fn advance_by(&mut self, displacement: u16) {
        self.ip = self.ip.wrapping_add(displacement)
    }
    pub fn reverse_by(&mut self, displacement: u16) {
        self.advance_by(u16::MAX - displacement)
    }
    pub fn jump(&mut self, address: u16) {
        self.ip = address;
    }
    pub fn peek_mem(&self, ahead: u16) -> u8 {
        self.memory[self.ip.wrapping_add(ahead) as usize]
    }
    pub fn peek_reg(&self, reg: usize) -> u8 {
        self.regs[reg]
    }
    pub fn poke_mem(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
    pub fn poke_reg(&mut self, reg: usize, value: u8) {
        self.regs[reg] = value;
    }
    pub fn load(&mut self, from: u16, to: usize) {
        self.regs[to] = self.memory[from as usize]
    }
    pub fn store(&mut self, from: usize, to: u16) {
        self.memory[to as usize] = self.regs[from]
    }
    pub fn add(&mut self, acc: usize, rhs: usize) {
        self.regs[acc] = self.regs[acc].wrapping_add(self.regs[rhs])
    }
    pub fn sub(&mut self, acc: usize, rhs: usize) {
        self.regs[acc] = self.regs[acc].wrapping_sub(self.regs[rhs]);
    }
    pub fn flag_set(&mut self) {
        self.flag = true;
    }
    pub fn flag_clear(&mut self) {
        self.flag = false;
    }
    pub fn branch_if(&mut self, displacement: u8) {
        if self.flag {
            self.flag_clear();
            self.advance_by(displacement as u16);
        }
    }
    pub fn less_than(&mut self, reg: usize, address: u16) {
        if self.regs[reg] < self.memory[address as usize] {
            self.flag_set()
        }
    }
    pub fn greater_than(&mut self, reg: usize, address: u16) {
        if self.regs[reg] > self.memory[address as usize] {
            self.flag_set()
        }
    }
    pub fn equal_to(&mut self, reg: usize, address: u16) {
        if self.regs[reg] > self.memory[address as usize] {
            self.flag_set()
        }
    }
}
use std::fs;
use std::num::Wrapping;

pub fn new_memory() -> Memory {
    Memory::new()
}

pub fn new_registers() -> Registers {
    Registers::new()
}

pub fn new_stack() -> Stack {
    Stack::new(MAX_STACK_SIZE)
}

const FONT: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

const RAM_SIZE: usize = 4 * 1024;
const ROM_OFFSET: usize = 0x200;

pub struct Memory {
    data: [u8; RAM_SIZE],
    pc: usize, // should this be separate?
}

impl Memory {
    fn new() -> Memory {
        Memory {
            data: [0; RAM_SIZE],
            pc: 0,
        }
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc
    }

    pub fn add_pc(&mut self, val: usize) {
        self.pc += val
    }

    pub fn load_rom(&mut self, rom_path: &String) {
        let rom_bytes = fs::read(rom_path).expect("unable to read rom file");

        if rom_bytes.len() > (self.data.len() - ROM_OFFSET) + 1 {
            panic!("rom is too large to fit into memory")
        }
        self.data[ROM_OFFSET..ROM_OFFSET + rom_bytes.len()].copy_from_slice(&rom_bytes);
        self.pc = ROM_OFFSET;
    }

    pub fn read(&mut self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn set(&mut self, addr: usize, val: u8) {
        self.data[addr] = val;
    }

    pub fn fetch_next(&mut self) -> Result<u16, &str> {
        // need to read 2 bytes
        if self.pc + 1 >= self.data.len() {
            return Err("exceeded memory");
        }
        // combine to hex operation
        let op = ((self.data[self.pc] as u16) << 8) | self.data[self.pc + 1] as u16;
        self.pc += 2;
        Ok(op)
    }
}

const NUM_REGISTERS: usize = 16;

pub struct Registers {
    // Registers are V0 through VF
    data: [u8; NUM_REGISTERS],
}

impl Registers {
    fn new() -> Registers {
        return Registers {
            data: [0; NUM_REGISTERS],
        };
    }

    pub fn get(&mut self, reg: u8) -> u8 {
        self.data[reg as usize]
    }

    pub fn set(&mut self, reg: u8, val: u8) {
        self.data[reg as usize] = val;
    }

    pub fn add(&mut self, reg: u8, val: u8) {
        let reg_val = self.data[reg as usize];
        self.data[reg as usize] = (Wrapping(reg_val) + Wrapping(val)).0;
    }

    pub fn sub(&mut self, reg: u8, val: u8) {
        let reg_val = self.data[reg as usize];
        self.data[reg as usize] = (Wrapping(reg_val) - Wrapping(val)).0;
    }
}

// Stack

static MAX_STACK_SIZE: usize = 100;

pub struct Stack {
    data: Vec<u16>,
    sp: usize,
    max_size: usize,
}

impl Stack {
    fn new(max_size: usize) -> Stack {
        Stack {
            data: Vec::with_capacity(max_size),
            sp: 0,
            max_size,
        }
    }

    fn push(&mut self, value: u16) {
        if self.sp >= self.max_size {
            // stack overflow!
            panic!("stack overflow")
        }
        if self.sp == self.data.len() {
            self.data.push(value);
        } else {
            self.data[self.sp] = value;
        }
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        if self.sp == 0 {
            // stack underflow!
            panic!("stack underflow")
        }
        self.sp -= 1;
        self.data[self.sp]
    }

    // TODO: push, pop, peek, clear?
}

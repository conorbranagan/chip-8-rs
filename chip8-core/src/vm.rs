#![allow(dead_code)]
use rand::Rng;
use std::fs;
use std::num::Wrapping;

use crate::display::Display;
use crate::instructions::Instruction;
use crate::memory::{Memory, Stack};

const NUM_REGISTERS: usize = 16;
const ROM_START: usize = 0x200;

struct Registers {
    data: [u8; NUM_REGISTERS],
    pc: usize,
}

impl Registers {
    fn new() -> Registers {
        return Registers {
            data: [0; NUM_REGISTERS],
            pc: ROM_START,
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

pub struct Chip8VM {
    memory: Memory,
    display: Display,
    registers: Registers,
    stack: Stack,
    index_register: usize,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8VM {
    pub fn new() -> Chip8VM {
        Chip8VM {
            memory: Memory::new(),
            display: Display::new(),
            registers: Registers::new(),
            stack: Stack::default(),
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load_rom(&mut self, rom_path: &String) {
        let rom_bytes = fs::read(rom_path).expect("unable to read rom file");
        for (i, b) in rom_bytes.iter().enumerate() {
            self.memory.write(ROM_START + i, *b);
        }
    }

    pub fn execute_next(&mut self) -> Result<u16, &str> {
        // need to read 2 bytes for the full instruction.
        let op1 = self.memory.read(self.registers.pc);
        let op2 = self.memory.read(self.registers.pc + 1);

        // combine to hex operation
        let op = ((op1 as u16) << 8) | op2 as u16;
        self.registers.pc += 2;

        let instr = Instruction::decode(op);
        self.execute(instr);

        Ok(op)
    }

    fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::Unknown(code) => {
                println!("Unknown instruction: {:#X}", code);
            }
            Instruction::ClearScreen => {
                println!("Executing ClearScreen");
                self.display.clear();
            }
            Instruction::ExitSubroutine => {
                println!("Executing ExitSubroutine");
                // Implement ExitSubroutine logic here
            }
            Instruction::Jump(addr) => {
                println!("Jumping to address {:#X}", addr);
                self.registers.pc = addr as usize;
            }
            Instruction::CallSubroutine(addr) => {
                println!("Calling subroutine at address {:#X}", addr);
                // Implement CallSubroutine logic here
            }
            Instruction::SkipValEqual(vx, val) => {
                println!("Skipping if register {} equals value {:#X}", vx, val);
                let vx_val = self.registers.get(vx);
                if val == vx_val {
                    self.registers.pc += 2;
                }
            }
            Instruction::SkipValNotEqual(vx, val) => {
                println!(
                    "Skipping if register {} does not equal value {:#X}",
                    vx, val
                );
                let vx_val: u8 = self.registers.get(vx);
                if val != vx_val {
                    self.registers.pc += 2;
                }
            }
            Instruction::SkipRegEqual(vx, vy) => {
                println!("Skipping if register {} equals register {}", vx, vy);
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                if vx_val == vy_val {
                    self.registers.pc += 2;
                }
            }
            Instruction::SetVal(vx, val) => {
                println!("Setting register {0} to value {1:} ({1:#X})", vx, val);
                self.registers.set(vx, val);
            }
            Instruction::AddVal(vx, val) => {
                println!("Adding value {:#X} to register {}", val, vx);
                self.registers.add(vx, val);
            }
            Instruction::SetReg(vx, vy) => {
                println!("Setting register {} to the value of register {}", vx, vy);
                let reg_val = self.registers.get(vy);
                self.registers.set(vx, reg_val);
            }
            Instruction::OR(vx, vy) => {
                println!("ORing register {} with register {}", vx, vy);
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                self.registers.set(vx, vx_val | vy_val);
            }
            Instruction::AND(vx, vy) => {
                println!("ANDing register {} with register {}", vx, vy);
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                self.registers.set(vx, vx_val & vy_val);
            }
            Instruction::XOR(vx, vy) => {
                println!("XORing register {} with register {}", vx, vy);
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                self.registers.set(vx, vx_val ^ vy_val);
            }
            Instruction::Add(vx, vy) => {
                println!("Adding register {} to register {}", vy, vx);
                let vy_val = self.registers.get(vy);
                self.registers.add(vx, vy_val);

                // Set carry flag for overflow
                let vx_val = self.registers.get(vx);
                if (vx_val as u32) + (vy_val as u32) > 255 {
                    self.registers.set(0xF, 1);
                } else {
                    self.registers.set(0xF, 0);
                }
            }
            Instruction::Sub(vx, vy) => {
                println!("Subtracting register {} from register {}", vy, vx);
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                self.registers.sub(vx, vy_val);

                // Set carry flag for underflow
                if vx_val > vy_val {
                    self.registers.set(0xF, 1);
                } else {
                    self.registers.set(0xF, 0);
                }
            }
            Instruction::ShiftRight(vx, _) => {
                println!("Shifting register {} right", vx);
                let reg_val = self.registers.get(vx);
                self.registers.set(vx, reg_val >> 1);
                self.registers.set(0xF, reg_val & 1);
            }
            Instruction::ShiftLeft(vx, _) => {
                println!("Shifting register {} left", vx);
                let reg_val = self.registers.get(vx);
                self.registers.set(vx, reg_val << 1);
                self.registers.set(0xF, (reg_val >> 7) & 1);
            }
            Instruction::SkipRegNotEqual(vx, vy) => {
                println!("Skipping if register {} does not equal register {}", vx, vy);
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                if vx_val != vy_val {
                    self.registers.pc += 2;
                }
            }
            Instruction::SetIndex(val) => {
                println!("Setting index register to {:#X}", val);
                self.index_register = val as usize;
            }
            Instruction::JumpOffset(val) => {
                println!("Jumping to address with offset {:#X}", val);
                self.registers.pc = (self.registers.get(0) as usize + val as usize) & 0xFFF;
            }
            Instruction::Random(vx, val) => {
                println!(
                    "Generating random number for register {} with mask {:#X}",
                    vx, val
                );
                let rand_val = rand::thread_rng().gen_range(0..=255) as u8;
                self.registers.set(vx, rand_val & val);
            }
            Instruction::Display(vx, vy, height) => {
                // Wrap coordinates around display.
                let x_coord = self.registers.get(vx);
                let y_coord = self.registers.get(vy);
                println!(
                    "Displaying sprite at ({}, {}) with height {}",
                    x_coord, y_coord, height
                );

                // VF starts at 0, will flip if any pixels are turned off.
                let mut vf = 0;
                let mut ireg: usize = self.index_register;

                for row in 0..height {
                    let sprite_byte: u8 = self.memory.read(ireg as usize);
                    let mut x_offset = 0;
                    for bit in (0..8).rev() {
                        let b: u8 = sprite_byte >> bit & 1;
                        let x = (x_coord + x_offset) as usize;
                        let y = (y_coord + row) as usize;
                        let p = self.display.get(x, y);

                        if b == 1 && p.unwrap() {
                            self.display.set(x, y, false);
                            vf = 1;
                        } else {
                            self.display.set(x, y, b == 1);
                        }
                        x_offset += 1;
                    }
                    ireg += 1;
                }
                self.registers.set(0xF, vf);
                self.display.render();
            }
            Instruction::SkipIfPressed(vx) => {
                println!("Skipping if key in register {} is pressed", vx);
                // Implement SkipIfPressed logic here
            }
            Instruction::SkipNotPressed(vx) => {
                println!("Skipping if key in register {} is not pressed", vx);
                // Implement SkipNotPressed logic here
            }
            Instruction::GetDelayTimer(vx) => {
                println!("Getting delay timer value into register {}", vx);
                self.registers.set(vx, self.delay_timer);
            }
            Instruction::SetDelayTimer(vx) => {
                println!("Setting delay timer to value in register {}", vx);
                self.delay_timer = self.registers.get(vx)
            }
            Instruction::SetSoundTimer(vx) => {
                println!("Setting sound timer to value in register {}", vx);
                self.sound_timer = self.registers.get(vx);
            }
            Instruction::AddToIndex(vx) => {
                println!("Adding register {} to index register", vx);
                self.index_register += self.registers.get(vx) as usize;
            }
            Instruction::GetKey(vx) => {
                println!("Waiting for key press to store in register {}", vx);
                // Implement GetKey logic here
            }
            Instruction::FontChar(vx) => {
                println!("Setting index to font character for register {}", vx);
                // Implement FontChar logic here
            }
            Instruction::BinDecConv(vx) => {
                let val = self.registers.get(vx);
                let (v1, v2, v3) = ((val / 100), (val / 10 % 10), (val % 10));
                let idx = self.index_register;
                self.memory.write(idx, v1);
                self.memory.write(idx + 1, v2);
                self.memory.write(idx + 2, v3);
                println!(
                    "Converting register {} to binary-coded decimal {} => ({}, {}, {})",
                    vx, val, v1, v2, v3
                );
            }
            Instruction::StoreMem(vx) => {
                println!("Storing registers 0 through {} into memory", vx);
                let mut addr = self.index_register;
                for vn in 0..=vx {
                    self.memory.write(addr, self.registers.get(vn));
                    addr += 1;
                }
            }
            Instruction::LoadMem(vx) => {
                println!("Loading memory into registers 0 through {}", vx);
                let mut addr = self.index_register;
                for vn in 0..=vx {
                    let val = self.memory.read(addr);
                    self.registers.set(vn, val);
                    addr += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_set_val() {
        let mut vm: Chip8VM = Chip8VM::new();
        vm.execute(Instruction::SetVal(1, 2));
        assert_eq!(vm.registers.get(1), 2);
    }
}

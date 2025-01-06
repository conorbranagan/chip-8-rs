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
    // Registers are V0 through VF
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
            Instruction::Jump { addr } => {
                println!("Jumping to address {:#X}", addr);
                self.registers.pc = addr as usize;
            }
            Instruction::CallSubroutine { addr } => {
                println!("Calling subroutine at address {:#X}", addr);
                // Implement CallSubroutine logic here
            }
            Instruction::SkipValEqual { reg, val } => {
                println!("Skipping if register {} equals value {:#X}", reg, val);
                let reg_val = self.registers.get(reg);
                if val == reg_val {
                    self.registers.pc += 2;
                }
            }
            Instruction::SkipValNotEqual { reg, val } => {
                println!(
                    "Skipping if register {} does not equal value {:#X}",
                    reg, val
                );
                let reg_val = self.registers.get(reg);
                if val != reg_val {
                    self.registers.pc += 2;
                }
            }
            Instruction::SkipRegEqual { reg1, reg2 } => {
                println!("Skipping if register {} equals register {}", reg1, reg2);
                // Implement SkipRegEqual logic here
                let reg1_val = self.registers.get(reg1);
                let reg2_val = self.registers.get(reg2);
                if reg1_val == reg2_val {
                    self.registers.pc += 2;
                }
            }
            Instruction::SetVal { reg, val } => {
                println!("Setting register {0} to value {1:} ({1:#X})", reg, val);
                self.registers.set(reg, val);
            }
            Instruction::AddVal { reg, val } => {
                println!("Adding value {:#X} to register {}", val, reg);
                self.registers.add(reg, val);
            }
            Instruction::SetReg { reg1, reg2 } => {
                println!(
                    "Setting register {} to the value of register {}",
                    reg1, reg2
                );
                let reg_val = self.registers.get(reg2);
                self.registers.set(reg1, reg_val);
            }
            Instruction::OR { reg1, reg2 } => {
                println!("ORing register {} with register {}", reg1, reg2);
                let reg1_val = self.registers.get(reg1);
                let reg2_val = self.registers.get(reg2);
                self.registers.set(reg1, reg1_val | reg2_val);
            }
            Instruction::AND { reg1, reg2 } => {
                println!("ANDing register {} with register {}", reg1, reg2);
                let reg1_val = self.registers.get(reg1);
                let reg2_val = self.registers.get(reg2);
                self.registers.set(reg1, reg1_val & reg2_val);
            }
            Instruction::XOR { reg1, reg2 } => {
                println!("XORing register {} with register {}", reg1, reg2);
                let reg1_val = self.registers.get(reg1);
                let reg2_val = self.registers.get(reg2);
                self.registers.set(reg1, reg1_val ^ reg2_val);
            }
            Instruction::Add { reg1, reg2 } => {
                println!("Adding register {} to register {}", reg2, reg1);
                let reg2_val = self.registers.get(reg2);
                self.registers.add(reg1, reg2_val);

                // need to set carry in case of overflow
                let reg1_val = self.registers.get(reg1);
                if (reg1_val as u32) + (reg2_val as u32) > 255 {
                    self.registers.set(0xF, 1);
                } else {
                    self.registers.set(0xF, 0);
                }
            }
            Instruction::Sub { reg1, reg2 } => {
                println!("Adding register {} to register {}", reg2, reg1);
                let reg1_val = self.registers.get(reg1);
                let reg2_val = self.registers.get(reg2);
                self.registers.sub(reg1, reg2_val);

                // set carry to handle underflow
                if reg1_val > reg2_val {
                    self.registers.set(0xF, 1);
                } else if reg1_val < reg2_val {
                    self.registers.set(0xF, 0);
                }
            }
            Instruction::ShiftRight { reg1, reg2: _ } => {
                // Use later implementation that ignores reg2
                println!("Shifting register {}", reg1);
                let reg_val = self.registers.get(reg1);
                self.registers.set(reg1, reg_val >> 1);
                self.registers.set(0xF, reg_val & 0x00F);
            }
            Instruction::ShiftLeft { reg1, reg2: _ } => {
                // Use later implementation that ignores reg2
                println!("Shifting register {}", reg1);
                let reg_val = self.registers.get(reg1);
                self.registers.set(reg1, reg_val << 1);
                self.registers.set(0xF, (reg_val >> 4) & 0x00F);
            }
            Instruction::SkipRegNotEqual { reg1, reg2 } => {
                println!(
                    "Skipping if register {} does not equal register {}",
                    reg1, reg2
                );
                let reg1_val = self.registers.get(reg1);
                let reg2_val = self.registers.get(reg2);
                if reg1_val != reg2_val {
                    self.registers.pc += 2;
                }
            }
            Instruction::SetIndex { val } => {
                println!("Setting index register to {:#X}", val);
                self.index_register = val as usize;
            }
            Instruction::JumpOffset { val } => {
                println!("Jumping to address with offset {:#X}", val);
                self.registers.pc = self.registers.get(0) as usize;
            }
            Instruction::Random { reg, val } => {
                println!(
                    "Generating random number for register {} with mask {:#X}",
                    reg, val
                );
                let rand_val = rand::thread_rng().gen_range(0..255) as u8;
                self.registers.set(0, rand_val & val);
            }
            Instruction::Display { reg1, reg2, height } => {
                // Wrap coordinates around display.
                let x_coord = self.registers.get(reg1);
                let y_coord = self.registers.get(reg2);
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
            Instruction::SkipIfPressed { reg } => {
                println!("Skipping if key in register {} is pressed", reg);
                // Implement SkipIfPressed logic here
            }
            Instruction::SkipNotPressed { reg } => {
                println!("Skipping if key in register {} is not pressed", reg);
                // Implement SkipNotPressed logic here
            }
            Instruction::GetDelayTimer { reg } => {
                println!("Getting delay timer value into register {}", reg);
                self.registers.set(reg, self.delay_timer);
            }
            Instruction::SetDelayTimer { reg } => {
                println!("Setting delay timer to value in register {}", reg);
                self.delay_timer = self.registers.get(reg)
            }
            Instruction::SetSoundTimer { reg } => {
                println!("Setting sound timer to value in register {}", reg);
                self.sound_timer = self.registers.get(reg);
            }
            Instruction::AddToIndex { reg } => {
                println!("Adding register {} to index register", reg);
                self.index_register += self.registers.get(reg) as usize;
            }
            Instruction::GetKey { reg } => {
                println!("Waiting for key press to store in register {}", reg);
                // Implement GetKey logic here
            }
            Instruction::FontChar { reg } => {
                println!("Setting index to font character for register {}", reg);
                // Implement FontChar logic here
            }
            Instruction::BinDecConv { reg } => {
                let val = self.registers.get(reg);
                let (v1, v2, v3) = ((val / 100), (val / 10 % 10), (val % 10));
                let idx = self.index_register;
                self.memory.write(idx, v1);
                self.memory.write(idx + 1, v2);
                self.memory.write(idx + 2, v3);
                println!(
                    "Converting register {} to binary-coded decimal {} => ({}, {}, {})",
                    reg, val, v1, v2, v3
                );
            }
            Instruction::StoreMem { to_reg } => {
                println!("Storing registers 0 through {} into memory", to_reg);
                let mut addr = self.index_register;
                for reg in 0..=to_reg {
                    self.memory.write(addr, self.registers.get(reg));
                    addr += 1;
                }
            }
            Instruction::LoadMem { to_reg } => {
                println!("Loading memory into registers 0 through {}", to_reg);
                let mut addr = self.index_register;
                for reg in 0..=to_reg {
                    let val = self.memory.read(addr);
                    self.registers.set(reg, val);
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
        vm.execute(Instruction::SetVal { reg: 1, val: 2 });
        assert_eq!(vm.registers.get(1), 2);
    }
}

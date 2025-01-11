use log::debug;
use rand::Rng;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use std::fs::{self, File};
use std::num::Wrapping;
use thiserror::Error;

use crate::display::Display;
use crate::instructions::Instruction;
use crate::keypad::Keypad;
use crate::memory::{Memory, Stack};

const NUM_REGISTERS: usize = 16;
const ROM_START: usize = 0x200;
const LOG_FILE: &str = "chip8-debug.log";

#[derive(Error, Debug)]
pub enum VMError {
    #[error("Unknown instruction: {0}")]
    UnknownInstruction(u16),

    #[error("Rom load error: {0}")]
    RomLoadFailure(String),

    #[error("Failed to initialize logging: {0}")]
    LogInitError(#[from] log::SetLoggerError),

    #[error("Failed to create log file: {0}")]
    FileCreationError(#[from] std::io::Error),
}

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
    keypad: Keypad,
    index_register: usize,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8VM {
    pub fn new() -> Result<Chip8VM, VMError> {
        let log_file = File::create(LOG_FILE)?;

        CombinedLogger::init(vec![WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            log_file,
        )])?;
        debug!("Initializing Chip8VM with log file: {}", LOG_FILE);

        Ok(Chip8VM {
            memory: Memory::new(),
            display: Display::new(),
            registers: Registers::new(),
            stack: Stack::default(),
            keypad: Keypad::new(),
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0,
        })
    }

    pub fn load_rom(&mut self, rom_path: &String) -> Result<(), VMError> {
        match fs::read(rom_path) {
            Ok(rom_bytes) => {
                for (i, b) in rom_bytes.iter().enumerate() {
                    self.memory.write(ROM_START + i, *b);
                }
                debug!("loaded {} into vm memory", rom_path);
                Ok(())
            }
            Err(e) => Err(VMError::RomLoadFailure(e.to_string())),
        }
    }

    pub fn run_cycle(&mut self) -> Result<(), VMError> {
        // need to read 2 bytes for the full instruction.
        let op1 = self.memory.read(self.registers.pc);
        let op2 = self.memory.read(self.registers.pc + 1);

        // combine to hex operation
        let op = ((op1 as u16) << 8) | op2 as u16;
        self.registers.pc += 2;

        let instr = Instruction::decode(op);
        self.execute(instr)
    }

    pub fn get_framebuffer(&mut self) -> &[bool] {
        self.display.get_framebuffer()
    }

    fn execute(&mut self, instr: Instruction) -> Result<(), VMError> {
        match instr {
            Instruction::Unknown(code) => {
                debug!("Unknown instruction: {:#X}", code);
                return Err(VMError::UnknownInstruction(code));
            }
            Instruction::ClearScreen => {
                debug!("Executing ClearScreen");
                self.display.clear();
            }
            Instruction::ExitSubroutine => {
                debug!("Executing ExitSubroutine");
                // TODO: Implement ExitSubroutine logic here
            }
            Instruction::Jump(addr) => {
                debug!("Jumping to address {:#X}", addr);
                self.registers.pc = addr as usize;
            }
            Instruction::CallSubroutine(addr) => {
                debug!("Calling subroutine at address {:#X}", addr);
                // TODO: Implement CallSubroutine logic here
            }
            Instruction::SkipValEqual(vx, val) => {
                debug!("Skipping if register {} equals value {:#X}", vx, val);
                let vx_val = self.registers.get(vx);
                if val == vx_val {
                    self.registers.pc += 2;
                }
            }
            Instruction::SkipValNotEqual(vx, val) => {
                debug!(
                    "Skipping if register {} does not equal value {:#X}",
                    vx, val
                );
                let vx_val: u8 = self.registers.get(vx);
                if val != vx_val {
                    self.registers.pc += 2;
                }
            }
            Instruction::SkipRegEqual(vx, vy) => {
                debug!("Skipping if register {} equals register {}", vx, vy);
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                if vx_val == vy_val {
                    self.registers.pc += 2;
                }
            }
            Instruction::SetVal(vx, val) => {
                debug!("Setting register {0} to value {1:} ({1:#X})", vx, val);
                self.registers.set(vx, val);
            }
            Instruction::AddVal(vx, val) => {
                debug!("Adding value {:#X} to register {}", val, vx);
                self.registers.add(vx, val);
            }
            Instruction::SetReg(vx, vy) => {
                debug!("Setting register {} to the value of register {}", vx, vy);
                let reg_val = self.registers.get(vy);
                self.registers.set(vx, reg_val);
            }
            Instruction::OR(vx, vy) => {
                debug!("ORing register {} with register {}", vx, vy);
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                self.registers.set(vx, vx_val | vy_val);
            }
            Instruction::AND(vx, vy) => {
                debug!("ANDing register {} with register {}", vx, vy);
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                self.registers.set(vx, vx_val & vy_val);
            }
            Instruction::XOR(vx, vy) => {
                debug!("XORing register {} with register {}", vx, vy);
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                self.registers.set(vx, vx_val ^ vy_val);
            }
            Instruction::Add(vx, vy) => {
                debug!("Adding register {} to register {}", vy, vx);
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
                debug!("Subtracting register {} from register {}", vy, vx);
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
                debug!("Shifting register {} right", vx);
                let reg_val = self.registers.get(vx);
                self.registers.set(vx, reg_val >> 1);
                self.registers.set(0xF, reg_val & 1);
            }
            Instruction::ShiftLeft(vx, _) => {
                debug!("Shifting register {} left", vx);
                let reg_val = self.registers.get(vx);
                self.registers.set(vx, reg_val << 1);
                self.registers.set(0xF, (reg_val >> 7) & 1);
            }
            Instruction::SkipRegNotEqual(vx, vy) => {
                debug!("Skipping if register {} does not equal register {}", vx, vy);
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                if vx_val != vy_val {
                    self.registers.pc += 2;
                }
            }
            Instruction::SetIndex(val) => {
                debug!("Setting index register to {:#X}", val);
                self.index_register = val as usize;
            }
            Instruction::JumpOffset(val) => {
                debug!("Jumping to address with offset {:#X}", val);
                self.registers.pc = (self.registers.get(0) as usize + val as usize) & 0xFFF;
            }
            Instruction::Random(vx, val) => {
                debug!(
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
                debug!(
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
            }
            Instruction::SkipIfPressed(vx) => {
                debug!("Skipping if key in register {} is pressed", vx);
                let vx_val: u8 = self.registers.get(vx);
                if vx_val == self.keypad.pressed() as u8 {
                    self.registers.pc += 2;
                }
            }
            Instruction::SkipNotPressed(vx) => {
                debug!("Skipping if key in register {} is not pressed", vx);
                let vx_val: u8 = self.registers.get(vx);
                if vx_val != self.keypad.pressed() as u8 {
                    self.registers.pc += 2;
                }
            }
            Instruction::GetDelayTimer(vx) => {
                debug!("Getting delay timer value into register {}", vx);
                self.registers.set(vx, self.delay_timer);
            }
            Instruction::SetDelayTimer(vx) => {
                debug!("Setting delay timer to value in register {}", vx);
                self.delay_timer = self.registers.get(vx)
            }
            Instruction::SetSoundTimer(vx) => {
                debug!("Setting sound timer to value in register {}", vx);
                self.sound_timer = self.registers.get(vx);
            }
            Instruction::AddToIndex(vx) => {
                debug!("Adding register {} to index register", vx);
                self.index_register += self.registers.get(vx) as usize;
            }
            Instruction::GetKey(vx) => {
                debug!("Waiting for key press to store in register {}", vx);
                // TODO: Implement GetKey logic here
                self.registers.set(vx, self.keypad.pressed() as u8);
            }
            Instruction::FontChar(vx) => {
                debug!("Setting index to font character for register {}", vx);
                // TODO: Implement FontChar logic here
            }
            Instruction::BinDecConv(vx) => {
                let val = self.registers.get(vx);
                let (v1, v2, v3) = ((val / 100), (val / 10 % 10), (val % 10));
                let idx = self.index_register;
                self.memory.write(idx, v1);
                self.memory.write(idx + 1, v2);
                self.memory.write(idx + 2, v3);
                debug!(
                    "Converting register {} to binary-coded decimal {} => ({}, {}, {})",
                    vx, val, v1, v2, v3
                );
            }
            Instruction::StoreMem(vx) => {
                debug!("Storing registers 0 through {} into memory", vx);
                let mut addr = self.index_register;
                for vn in 0..=vx {
                    self.memory.write(addr, self.registers.get(vn));
                    addr += 1;
                }
            }
            Instruction::LoadMem(vx) => {
                debug!("Loading memory into registers 0 through {}", vx);
                let mut addr = self.index_register;
                for vn in 0..=vx {
                    let val = self.memory.read(addr);
                    self.registers.set(vn, val);
                    addr += 1;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_set_val() {
        let vm_result = Chip8VM::new();
        assert!(vm_result.is_ok());
        let mut vm = vm_result.unwrap();
        let result = vm.execute(Instruction::SetVal(1, 2));
        assert!(result.is_ok());
        assert_eq!(vm.registers.get(1), 2);
    }
}

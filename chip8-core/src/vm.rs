use log::debug;
use rand::Rng;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use std::fs::{self, File};
use std::ops::{Index, IndexMut};
use thiserror::Error;

use crate::display::Display;
use crate::instructions::Instruction;
use crate::keypad::{Key, KeyState, Keypad};
use crate::memory::{Memory, Stack};

const NUM_REGISTERS: usize = 16;
const ROM_START: usize = 0x200;
const LOG_FILE: &str = "chip8-debug.log";

#[derive(Error, Debug)]
pub enum VMError {
    #[error("Unknown instruction: {0}")]
    UnknownInstruction(u16),

    #[error("Unknown key: {0}")]
    UnknownKey(u8),

    #[error("Rom load error: {0}")]
    RomLoadFailure(String),

    #[error("Failed to initialize logging: {0}")]
    LogInitError(#[from] log::SetLoggerError),

    #[error("Failed to create log file: {0}")]
    FileCreationError(#[from] std::io::Error),

    #[error("Stack underflow")]
    StackUnderflow(),

    #[error("Stack overflow")]
    StackOverflow(),
}

struct Registers {
    data: [u8; NUM_REGISTERS],
    pc: usize,
}

type RegNum = u8;

impl Registers {
    fn new() -> Registers {
        return Registers {
            data: [0; NUM_REGISTERS],
            pc: ROM_START,
        };
    }
}

impl Index<RegNum> for Registers {
    type Output = u8;
    fn index(&self, index: RegNum) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl IndexMut<RegNum> for Registers {
    fn index_mut(&mut self, index: RegNum) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

pub struct Chip8VM {
    memory: Memory,
    display: Display,
    registers: Registers,
    stack: Stack,
    keypad: Keypad,
    index_register: usize,
    key_wait: Option<RegNum>,
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
            key_wait: None,
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
        debug!("execute instruction @ {:#X}", self.registers.pc);

        // combine to hex operation
        let op = ((op1 as u16) << 8) | op2 as u16;
        self.registers.pc += 2;

        let instr = Instruction::decode(op);
        self.execute(instr)
    }

    pub fn get_framebuffer(&mut self) -> &[bool] {
        self.display.get_framebuffer()
    }

    pub fn handle_key(&mut self, key_code: u8, is_pressed: bool) {
        if let Ok(key) = Key::try_from(key_code) {
            self.keypad[key] = if is_pressed {
                KeyState::Pressed
            } else {
                KeyState::NotPressed
            };
        }
        if let Some(vx) = self.key_wait {
            self.registers[vx] = key_code;
            self.key_wait = None;
        }
    }

    fn execute(&mut self, instr: Instruction) -> Result<(), VMError> {
        // When we're waiting on a key we won't execute any more instructions
        // until handle_key is called and `key_wait` gets reset.
        if self.key_wait.is_some() {
            return Ok(());
        }

        use Instruction::*;
        match instr {
            Unknown(code) => {
                debug!("Unknown instruction: {:#X}", code);
                return Err(VMError::UnknownInstruction(code));
            }
            ClearScreen => {
                debug!("Executing ClearScreen");
                self.display.clear();
            }
            ExitSubroutine => {
                debug!("Exit subroutine");
                if let Ok(addr) = self.stack.pop() {
                    self.registers.pc = addr as usize;
                }
            }
            Jump(addr) => {
                debug!("Jumping to address {:#X}", addr);
                self.registers.pc = addr as usize;
            }
            CallSubroutine(addr) => {
                debug!("Calling subroutine at address {:#X}", addr);
                if self.stack.push(self.registers.pc as u16).is_ok() {
                    self.registers.pc = addr as usize;
                } else {
                    panic!("alksfjalksjfalksjf");
                }
            }
            SkipValEqual(vx, val) => {
                debug!("Skipping if register {} equals value {:#X}", vx, val);
                if val == self.registers[vx] {
                    self.registers.pc += 2;
                }
            }
            SkipValNotEqual(vx, val) => {
                debug!("Skipping if register {} != {:#X}", vx, val);
                if val != self.registers[vx] {
                    self.registers.pc += 2;
                }
            }
            SkipRegEqual(vx, vy) => {
                debug!("Skipping if register {} equals register {}", vx, vy);
                if self.registers[vx] == self.registers[vy] {
                    self.registers.pc += 2;
                }
            }
            SetVal(vx, val) => {
                debug!("Setting register {0} to value {1:} ({1:#X})", vx, val);
                self.registers[vx] = val;
            }
            AddVal(vx, val) => {
                debug!("Adding value {:#X} to register {}", val, vx);
                self.registers[vx] = self.registers[vx].wrapping_add(val);
            }
            SetReg(vx, vy) => {
                debug!("Setting register {} to the value of register {}", vx, vy);
                self.registers[vx] = self.registers[vy];
            }
            OR(vx, vy) => {
                debug!("ORing register {} with register {}", vx, vy);
                let vx_val = self.registers[vx];
                let vy_val = self.registers[vy];
                self.registers[vx] = vx_val | vy_val;
            }
            AND(vx, vy) => {
                debug!("ANDing register {} with register {}", vx, vy);
                self.registers[vx] &= self.registers[vy];
            }
            XOR(vx, vy) => {
                debug!("XORing register {} with register {}", vx, vy);
                self.registers[vx] ^= self.registers[vy];
            }
            Add(vx, vy) => {
                debug!("Adding register {} to register {}", vy, vx);
                let vy_val = self.registers[vy];
                self.registers[vx] = self.registers[vx].wrapping_add(vy_val);

                // Set carry flag for overflow
                let vx_val = self.registers[vx];
                if (vx_val as u32) + (vy_val as u32) > 255 {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }
            }
            SubLeft(vx, vy) => {
                let vx_val = self.registers[vx];
                let vy_val = self.registers[vy];
                self.registers[vx] = vx_val.wrapping_sub(vy_val);
                // Set carry flag for underflow
                if vx_val > vy_val {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }
            }
            SubRight(vx, vy) => {
                let vx_val = self.registers[vx];
                let vy_val = self.registers[vy];
                self.registers[vx] = vy_val.wrapping_sub(vx_val);
                // Set carry flag for underflow
                if vy_val > vx_val {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }
            }
            ShiftRight(vx, _) => {
                debug!("Shifting register {} right", vx);
                let reg_val = self.registers[vx];
                self.registers[vx] = reg_val >> 1;
                self.registers[0xF] = reg_val & 1;
            }
            ShiftLeft(vx, _) => {
                debug!("Shifting register {} left", vx);
                let reg_val = self.registers[vx];
                self.registers[vx] = reg_val << 1;
                self.registers[0xF] = (reg_val >> 7) & 1;
            }
            SkipRegNotEqual(vx, vy) => {
                debug!("Skipping if register {} does not equal register {}", vx, vy);
                let vx_val = self.registers[vx];
                let vy_val = self.registers[vy];
                if vx_val != vy_val {
                    self.registers.pc += 2;
                }
            }
            SetIndex(val) => {
                debug!("Setting index register to {:#X}", val);
                self.index_register = val as usize;
            }
            JumpOffset(val) => {
                debug!("Jumping to address with offset {:#X}", val);
                self.registers.pc = (self.registers[0x0] as usize + val as usize) & 0xFFF;
            }
            Random(vx, val) => {
                debug!(
                    "Generating random number for register {} with mask {:#X}",
                    vx, val
                );
                let rand_val = rand::thread_rng().gen_range(0..=255) as u8;
                self.registers[vx] = rand_val & val;
            }
            Display(vx, vy, height) => {
                // Wrap coordinates around display.
                let x_coord = self.registers[vx];
                let y_coord = self.registers[vy];
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
                self.registers[0xF] = vf;
            }
            SkipIfPressed(vx) => {
                debug!("Skipping if key in register {} is pressed", vx);
                let vx_val: u8 = self.registers[vx];
                let key: Key = Key::try_from(vx_val)?;
                if self.keypad[key] == KeyState::Pressed {
                    self.registers.pc += 2;
                }
            }
            SkipNotPressed(vx) => {
                debug!("Skipping if key in register {} is not pressed", vx);
                let vx_val: u8 = self.registers[vx];
                let key: Key = Key::try_from(vx_val)?;
                if self.keypad[key] == KeyState::Pressed {
                    self.registers.pc += 2;
                }
            }
            GetDelayTimer(vx) => {
                debug!("Getting delay timer value into register {}", vx);
                self.registers[vx] = self.delay_timer;
            }
            SetDelayTimer(vx) => {
                debug!("Setting delay timer to value in register {}", vx);
                self.delay_timer = self.registers[vx];
            }
            SetSoundTimer(vx) => {
                debug!("Setting sound timer to value in register {}", vx);
                self.sound_timer = self.registers[vx];
            }
            AddToIndex(vx) => {
                debug!("Adding register {} to index register", vx);
                self.index_register += self.registers[vx] as usize;
            }
            GetKey(vx) => {
                debug!("Waiting for key press to store in register {}", vx);
                self.key_wait = Some(vx);
            }
            FontChar(vx) => {
                debug!("Setting index to font character for register {}", vx);
                // TODO: Implement FontChar logic here
            }
            BinDecConv(vx) => {
                let val = self.registers[vx];
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
            StoreMem(vx) => {
                debug!("Storing registers 0 through {} into memory", vx);
                let mut addr = self.index_register;
                for vn in 0..=vx {
                    self.memory.write(addr, self.registers[vn]);
                    addr += 1;
                }
            }
            LoadMem(vx) => {
                debug!("Loading memory into registers 0 through {}", vx);
                let mut addr = self.index_register;
                for vn in 0..=vx {
                    let val = self.memory.read(addr);
                    self.registers[vn] = val;
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
    fn test_registers_8bits() {
        let vm_result = Chip8VM::new();
        assert!(vm_result.is_ok());
        let mut vm = vm_result.unwrap();

        // https://github.com/Timendus/chip8-test-suite/blob/main/src/tests/3-corax+.8o#L351
        // no overflow
        assert!(vm.execute(Instruction::SetVal(6, 255)).is_ok());
        assert_eq!(vm.registers[6], 255);
        assert!(vm.execute(Instruction::AddVal(6, 10)).is_ok());
        assert_eq!(vm.registers[6], 9);
        assert!(vm.execute(Instruction::ShiftRight(6, 6)).is_ok());
        assert_eq!(vm.registers[6], 4);
        assert!(vm.execute(Instruction::SetVal(6, 255)).is_ok());
        assert_eq!(vm.registers[6], 255);
        assert!(vm.execute(Instruction::SetVal(0, 10)).is_ok());
        assert_eq!(vm.registers[0], 10);
        assert!(vm.execute(Instruction::Add(6, 0)).is_ok());
        assert_eq!(vm.registers[6], 9);
        assert!(vm.execute(Instruction::ShiftRight(6, 6)).is_ok());
        assert_eq!(vm.registers[6], 4);

        // do not retain bits
        assert!(vm.execute(Instruction::SetVal(6, 255)).is_ok());
        assert_eq!(vm.registers[6], 255);
        assert!(vm.execute(Instruction::ShiftLeft(6, 6)).is_ok());
        assert!(vm.execute(Instruction::ShiftRight(6, 6)).is_ok());
        assert_eq!(vm.registers[6], 127);
        assert!(vm.execute(Instruction::ShiftRight(6, 6)).is_ok());
        assert!(vm.execute(Instruction::ShiftLeft(6, 6)).is_ok());
        assert_eq!(vm.registers[6], 126);

        assert!(vm.execute(Instruction::SetVal(6, 5)).is_ok());
        assert_eq!(vm.registers[6], 5);
        assert!(vm.execute(Instruction::SetVal(0, 10)).is_ok());
        assert_eq!(vm.registers[0], 10);
        assert!(vm.execute(Instruction::SubLeft(6, 0)).is_ok());
        assert_eq!(vm.registers[6], 251);

        assert!(vm.execute(Instruction::SetVal(6, 5)).is_ok());
        assert_eq!(vm.registers[6], 5);
        assert!(vm.execute(Instruction::SubLeft(6, 0)).is_ok());
        assert_eq!(vm.registers[6], 251);
        assert!(vm.execute(Instruction::SetVal(6, 5)).is_ok());
        assert_eq!(vm.registers[6], 5);
        assert!(vm.execute(Instruction::SubRight(0, 6)).is_ok());
        assert_eq!(vm.registers[0], 251);
    }
}

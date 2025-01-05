#![allow(dead_code)]

use std::env;
use std::fs;

// Sound & Display

const DISPLAY_WIDTH: u8 = 64;
const DISPLAY_HEIGHT: u8 = 32;

struct Display {
    pixels: [[bool; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
}

impl Display {
    fn new() -> Display {
        Display {
            pixels: [[false; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
        }
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        if y >= self.pixels.len() || y >= self.pixels[y].len() {
            // Cut off bytes outside of the display.
            return;
        }
        self.pixels[y][x] = val;
    }

    fn get(&mut self, x: usize, y: usize) -> Result<bool, &str> {
        if y >= self.pixels.len() || y >= self.pixels[y].len() {
            return Err("pixel out of range");
        }
        Ok(self.pixels[y][x])
    }

    fn clear(&mut self) {
        self.pixels = [[false; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize];
    }

    fn render(&mut self) {
        for row in self.pixels.iter_mut() {
            for v in row.iter() {
                if *v {
                    print!("█")
                } else {
                    print!(" ")
                }
            }
            print!("\n");
        }
    }
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

// Memory

const RAM_SIZE: usize = 4 * 1024;
const ROM_OFFSET: usize = 0x200;

struct Memory {
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

    fn load_rom(&mut self, rom_path: &String) {
        let rom_bytes = fs::read(rom_path).expect("unable to read rom file");

        if rom_bytes.len() > (self.data.len() - ROM_OFFSET) + 1 {
            panic!("rom is too large to fit into memory")
        }
        self.data[ROM_OFFSET..ROM_OFFSET + rom_bytes.len()].copy_from_slice(&rom_bytes);
        self.pc = ROM_OFFSET;
    }

    fn read(&mut self, addr: usize) -> u8 {
        self.data[addr]
    }

    fn fetch_next(&mut self) -> Result<u16, &str> {
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

struct Registers {
    // Registers are V0 through VF
    data: [u8; NUM_REGISTERS],
}

impl Registers {
    fn new() -> Registers {
        return Registers {
            data: [0; NUM_REGISTERS],
        };
    }

    fn get(&mut self, register_num: u8) -> u8 {
        self.data[register_num as usize]
    }

    fn set(&mut self, register_num: u8, val: u8) {
        self.data[register_num as usize] = val;
    }

    fn add(&mut self, register_num: u8, val: u8) {
        self.data[register_num as usize] += val;
    }
}

// Stack

static MAX_STACK_SIZE: usize = 100;

struct Stack {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: chip8 <path/to/rom.ch8>");
        return;
    }

    let mut chip8 = Chip8::new();
    let rom_path = args.get(1).unwrap();
    chip8.load_rom(rom_path);
    println!("Loaded {} into memory", rom_path);

    let mut cnt = 1;
    loop {
        // fetch()
        let op = chip8.fetch_next().unwrap();

        // FIXME
        if op == 0x0 {
            break;
        }

        let instr = decode(op);

        // execute()
        chip8.execute(instr);

        cnt += 1;
        if cnt > 25 {
            break;
        }
    }
}

fn decode(instr: u16) -> Instr {
    println!("{:#X}", instr);
    let opcode = (instr & 0xF000) >> 12;

    use Instr::*;
    match opcode {
        0x0 => match (instr & 0x00FF) as u8 {
            0xE0 => ClearScreen,
            0xEE => ExitSubroutine,
            _ => Unknown(instr),
        },
        0x1 => Jump {
            addr: instr & 0x0FFF,
        },
        0x2 => CallSubroutine {
            addr: instr & 0x0FFF,
        },
        0x3 => SkipValEqual {
            reg: d_reg1(instr),
            val: d_val(instr),
        },
        0x4 => SkipValNotEqual {
            reg: d_reg1(instr),
            val: d_val(instr),
        },
        0x5 => SkipRegEqual {
            reg1: d_reg1(instr),
            reg2: d_reg2(instr),
        },
        0x6 => SetVal {
            reg: d_reg1(instr),
            val: d_val(instr),
        },
        0x7 => AddVal {
            reg: d_reg1(instr),
            val: d_val(instr),
        },
        0x8 => match instr & 0x000F {
            0x0 => SetReg {
                reg1: d_reg1(instr),
                reg2: d_reg2(instr),
            },
            0x1 => OR {
                reg1: d_reg1(instr),
                reg2: d_reg2(instr),
            },
            0x2 => AND {
                reg1: d_reg1(instr),
                reg2: d_reg2(instr),
            },
            0x3 => XOR {
                reg1: d_reg1(instr),
                reg2: d_reg2(instr),
            },
            0x4 => Add {
                reg1: d_reg1(instr),
                reg2: d_reg2(instr),
            },
            0x5 => Sub {
                reg1: d_reg1(instr),
                reg2: d_reg2(instr),
            },
            0x6 => Shift {
                reg1: d_reg1(instr),
                reg2: d_reg2(instr),
            },
            0x7 => Sub {
                reg1: d_reg2(instr),
                reg2: d_reg1(instr),
            },
            0xE => Shift {
                reg1: d_reg2(instr),
                reg2: d_reg1(instr),
            },
            _ => Unknown(instr),
        },
        0x9 => SkipRegNotEqual {
            reg1: d_reg1(instr),
            reg2: d_reg2(instr),
        },
        0xA => SetIndex {
            val: d_val16(instr),
        },
        0xB => JumpOffset {
            val: d_val16(instr),
        },
        0xC => Random {
            reg: d_reg1(instr),
            val: d_val(instr),
        },
        0xD => Display {
            reg1: d_reg1(instr),
            reg2: d_reg2(instr),
            height: (instr & 0x000F) as u8,
        },
        0xE => match instr & 0x00FF {
            0x9E => SkipIfPressed { reg: d_reg1(instr) },
            0xA1 => SkipNotPressed { reg: d_reg1(instr) },
            _ => Unknown(instr),
        },
        0xF => match instr & 0x00FF {
            0x07 => GetDelayTimer { reg: d_reg1(instr) },
            0x15 => SetDelayTimer { reg: d_reg1(instr) },
            0x18 => SetSoundTimer { reg: d_reg1(instr) },
            0x1E => AddToIndex { reg: d_reg1(instr) },
            0x0A => GetKey { reg: d_reg1(instr) },
            0x29 => FontChar { reg: d_reg1(instr) },
            0x33 => BinDecConv { reg: d_reg1(instr) },
            0x55 => StoreMem { reg: d_reg1(instr) },
            0x65 => LoadMem { reg: d_reg1(instr) },
            _ => Unknown(instr),
        },
        _ => Unknown(instr),
    }
}

fn d_val(instr: u16) -> u8 {
    (instr & 0x00FF) as u8
}

fn d_val16(instr: u16) -> u16 {
    instr & 0x0FFF
}

fn d_reg1(instr: u16) -> u8 {
    ((instr & 0x0F00) >> 8) as u8
}

fn d_reg2(instr: u16) -> u8 {
    ((instr & 0x00F0) >> 4) as u8
}

#[derive(Debug)]
enum Instr {
    Unknown(u16),
    ClearScreen,                                // 00E0
    ExitSubroutine,                             // 00EE
    Jump { addr: u16 },                         // 1NNN
    CallSubroutine { addr: u16 },               // 2NNN
    SkipValEqual { reg: u8, val: u8 },          // 3XNN
    SkipValNotEqual { reg: u8, val: u8 },       // 4XNN
    SkipRegEqual { reg1: u8, reg2: u8 },        // 5XY0
    SetVal { reg: u8, val: u8 },                // 6XNN
    AddVal { reg: u8, val: u8 },                // 7XNN
    SetReg { reg1: u8, reg2: u8 },              // 8XY0
    OR { reg1: u8, reg2: u8 },                  // 8XY1
    AND { reg1: u8, reg2: u8 },                 // 8XY2
    XOR { reg1: u8, reg2: u8 },                 // 8XY3
    Add { reg1: u8, reg2: u8 },                 // 8XY4
    Sub { reg1: u8, reg2: u8 },                 // 8XY5, 8XY7
    Shift { reg1: u8, reg2: u8 },               // 8XY6, 8XYE
    SkipRegNotEqual { reg1: u8, reg2: u8 },     // 9XY0
    SetIndex { val: u16 },                      // ANNN
    JumpOffset { val: u16 },                    // BNNN
    Random { reg: u8, val: u8 },                // CXNN
    Display { reg1: u8, reg2: u8, height: u8 }, // DXYN
    SkipIfPressed { reg: u8 },                  // EX9E
    SkipNotPressed { reg: u8 },                 // EXA1
    GetDelayTimer { reg: u8 },                  // FX07
    SetDelayTimer { reg: u8 },                  // FX15
    SetSoundTimer { reg: u8 },                  // FX18
    AddToIndex { reg: u8 },                     // FX1E
    GetKey { reg: u8 },                         // FX0A
    FontChar { reg: u8 },                       // FX29
    BinDecConv { reg: u8 },                     // FX33
    StoreMem { reg: u8 },                       // FX55
    LoadMem { reg: u8 },                        // FX65
}

struct Chip8 {
    memory: Memory,
    display: Display,
    registers: Registers,
    stack: Stack,
    index_register: usize,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    fn new() -> Chip8 {
        Chip8 {
            memory: Memory::new(),
            display: Display::new(),
            registers: Registers::new(),
            stack: Stack::new(MAX_STACK_SIZE),
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    fn load_rom(&mut self, rom_path: &String) {
        self.memory.load_rom(rom_path);
    }

    fn fetch_next(&mut self) -> Result<u16, &str> {
        self.memory.fetch_next()
    }

    fn execute(&mut self, instr: Instr) {
        use Instr::*;
        match instr {
            Unknown(code) => {
                println!("Unknown instruction: {:#X}", code);
            }
            ClearScreen => {
                println!("Executing ClearScreen");
                self.display.clear();
            }
            ExitSubroutine => {
                println!("Executing ExitSubroutine");
                // Implement ExitSubroutine logic here
            }
            Jump { addr } => {
                println!("Jumping to address {:#X}", addr);
                self.memory.pc = addr as usize;
            }
            CallSubroutine { addr } => {
                println!("Calling subroutine at address {:#X}", addr);
                // Implement CallSubroutine logic here
            }
            SkipValEqual { reg, val } => {
                println!("Skipping if register {} equals value {:#X}", reg, val);
                // Implement SkipValEqual logic here
            }
            SkipValNotEqual { reg, val } => {
                println!(
                    "Skipping if register {} does not equal value {:#X}",
                    reg, val
                );
                // Implement SkipValNotEqual logic here
            }
            SkipRegEqual { reg1, reg2 } => {
                println!("Skipping if register {} equals register {}", reg1, reg2);
                // Implement SkipRegEqual logic here
            }
            SetVal { reg, val } => {
                println!("Setting register {0} to value {1:} ({1:#X})", reg, val);
                self.registers.set(reg, val);
            }
            AddVal { reg, val } => {
                println!("Adding value {:#X} to register {}", val, reg);
                self.registers.add(reg, val);
            }
            SetReg { reg1, reg2 } => {
                println!(
                    "Setting register {} to the value of register {}",
                    reg1, reg2
                );
                // Implement SetReg logic here
            }
            OR { reg1, reg2 } => {
                println!("ORing register {} with register {}", reg1, reg2);
                // Implement OR logic here
            }
            AND { reg1, reg2 } => {
                println!("ANDing register {} with register {}", reg1, reg2);
                // Implement AND logic here
            }
            XOR { reg1, reg2 } => {
                println!("XORing register {} with register {}", reg1, reg2);
                // Implement XOR logic here
            }
            Add { reg1, reg2 } => {
                println!("Adding register {} to register {}", reg2, reg1);
                // Implement Add logic here
            }
            Sub { reg1, reg2 } => {
                println!("Subtracting register {} from register {}", reg2, reg1);
                // Implement Sub logic here
            }
            Shift { reg1, reg2 } => {
                println!("Shifting register {} with register {}", reg1, reg2);
                // Implement Shift logic here
            }
            SkipRegNotEqual { reg1, reg2 } => {
                println!(
                    "Skipping if register {} does not equal register {}",
                    reg1, reg2
                );
                // Implement SkipRegNotEqual logic here
            }
            SetIndex { val } => {
                println!("Setting index register to {:#X}", val);
                // Implement SetIndex logic here
                self.index_register = val as usize;
            }
            JumpOffset { val } => {
                println!("Jumping to address with offset {:#X}", val);
                // Implement JumpOffset logic here
            }
            Random { reg, val } => {
                println!(
                    "Generating random number for register {} with mask {:#X}",
                    reg, val
                );
                // Implement Random logic here
            }
            Display { reg1, reg2, height } => {
                // Wrap coordinates around display.
                let x_coord = self.registers.get(reg1) & (DISPLAY_WIDTH - 1);
                let y_coord = self.registers.get(reg2) & (DISPLAY_HEIGHT - 1);
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
            SkipIfPressed { reg } => {
                println!("Skipping if key in register {} is pressed", reg);
                // Implement SkipIfPressed logic here
            }
            SkipNotPressed { reg } => {
                println!("Skipping if key in register {} is not pressed", reg);
                // Implement SkipNotPressed logic here
            }
            GetDelayTimer { reg } => {
                println!("Getting delay timer value into register {}", reg);
                // Implement GetDelayTimer logic here
            }
            SetDelayTimer { reg } => {
                println!("Setting delay timer to value in register {}", reg);
                // Implement SetDelayTimer logic here
            }
            SetSoundTimer { reg } => {
                println!("Setting sound timer to value in register {}", reg);
                // Implement SetSoundTimer logic here
            }
            AddToIndex { reg } => {
                println!("Adding register {} to index register", reg);
                // Implement AddToIndex logic here
            }
            GetKey { reg } => {
                println!("Waiting for key press to store in register {}", reg);
                // Implement GetKey logic here
            }
            FontChar { reg } => {
                println!("Setting index to font character for register {}", reg);
                // Implement FontChar logic here
            }
            BinDecConv { reg } => {
                println!("Converting register {} to binary-coded decimal", reg);
                // Implement BinDecConv logic here
            }
            StoreMem { reg } => {
                println!("Storing registers 0 through {} into memory", reg);
                // Implement StoreMem logic here
            }
            LoadMem { reg } => {
                println!("Loading memory into registers 0 through {}", reg);
                // Implement LoadMem logic here
            }
        }
    }
}

macro_rules! decode_tests {
    ( $($label:ident : $inp:expr, $pat:pat,)* ) => {
    $(
        #[test]
        fn $label() {
            assert!(matches!(decode($inp), $pat), "got {:?}", decode($inp))
        }
    )*
    }
}

decode_tests! {
    t1:  0x00E0, Instr::ClearScreen,
    t2:  0x00EE, Instr::ExitSubroutine,
    t3:  0x1EAF, Instr::Jump { addr: 0x0EAF },
    t4:  0x2FEA, Instr::CallSubroutine{ addr: 0x0FEA},
    t5:  0x324B, Instr::SkipValEqual{reg: 0x2, val: 0x4B},
    t6:  0x4401, Instr::SkipValNotEqual{reg: 0x4, val: 0x01},
    t7:  0x5230, Instr::SkipRegEqual{reg1: 0x2, reg2: 0x3},
    t8:  0x62F4, Instr::SetVal{ reg: 2, val: 0xF4},
    t9:  0x713F, Instr::AddVal{ reg: 1, val: 0x3F},
    t10: 0x8240, Instr::SetReg{ reg1: 2, reg2: 4},
    t11: 0x8231, Instr::OR{ reg1: 2, reg2: 3},
    t12: 0x8232, Instr::AND{ reg1: 2, reg2: 3},
    t13: 0x8233, Instr::XOR{ reg1: 2, reg2: 3},
    t14: 0x8234, Instr::Add{ reg1: 2, reg2: 3},
    t15: 0x8235, Instr::Sub{ reg1: 2, reg2: 3},
    t16: 0x8236, Instr::Shift{ reg1: 2, reg2: 3},
    t17: 0x9230, Instr::SkipRegNotEqual{ reg1: 2, reg2: 3},
    t18: 0xA123, Instr::SetIndex{ val: 0x0123},
    t19: 0xB456, Instr::JumpOffset{ val: 0x0456},
    t20: 0xC3A5, Instr::Random{ reg: 3, val: 0xA5},
    t21: 0xD125, Instr::Display{ reg1: 1, reg2: 2, height: 0x5},
    t22: 0xE19E, Instr::SkipIfPressed{ reg: 1},
    t23: 0xE1A1, Instr::SkipNotPressed{ reg: 1},
    t24: 0xF107, Instr::GetDelayTimer{ reg: 1},
    t25: 0xF215, Instr::SetDelayTimer{ reg: 2},
    t26: 0xF318, Instr::SetSoundTimer{ reg: 3},
    t27: 0xF41E, Instr::AddToIndex{ reg: 4},
    t28: 0xF50A, Instr::GetKey{ reg: 5},
    t29: 0xF629, Instr::FontChar{ reg: 6},
    t30: 0xF733, Instr::BinDecConv{ reg: 7},
    t31: 0xF855, Instr::StoreMem{ reg: 8},
    t32: 0xF965, Instr::LoadMem{ reg: 9},
}

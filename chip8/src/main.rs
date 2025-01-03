#![allow(dead_code)]

use std::env;
use std::fs;

// Sound & Display

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

struct Display {
    pixels: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Display {
    fn new() -> Display {
        Display {
            pixels: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }

    fn clear(&mut self) {
        self.pixels = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
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

static DELAY_TIMER: u8 = 0;
static SOUND_TIMER: u8 = 0;

// Memory

const RAM_SIZE: usize = 4 * 1024;

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

        // Load rom starting at Address 200 per doc.
        let start = 100;
        if rom_bytes.len() > (self.data.len() - start) + 1 {
            panic!("rom is too large to fit into memory")
        }
        self.data[start..start + rom_bytes.len()].copy_from_slice(&rom_bytes);
        self.pc = start;
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

struct IndexRegister {
    val: u8,
}

impl IndexRegister {
    fn new() -> IndexRegister {
        IndexRegister { val: 0 }
    }

    fn set(&mut self, val: u8) {
        self.val = val;
    }
}

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

    fn set(&mut self, register_num: usize, val: u8) {
        self.data[register_num] = val;
    }

    fn add(&mut self, register_num: usize, val: u8) {
        self.data[register_num] += val;
    }
}

// Stack

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

    let mut memory = Memory::new();
    let display = Display::new();
    let stack = Stack::new(16);
    let index_register = IndexRegister::new();

    let rom_path = args.get(1).unwrap();
    memory.load_rom(rom_path);
    println!("Loaded {} into memory", rom_path);

    loop {
        // fetch()
        let op = memory.fetch_next().unwrap();

        // FIXME
        if op == 0x0 {
            break;
        }

        let instr = decode(op);
        println!("{:?}", instr)

        // execute()
    }
}

fn decode(instr: u16) -> Instr {
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
            pixels: (instr & 0x000F) as u8,
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
    Display { reg1: u8, reg2: u8, pixels: u8 }, // DXYN
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
    t21: 0xD125, Instr::Display{ reg1: 1, reg2: 2, pixels: 0x5},
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

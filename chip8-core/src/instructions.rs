#[derive(Debug)]
pub enum Instruction {
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
    ShiftRight { reg1: u8, reg2: u8 },          // 8XY6
    ShiftLeft { reg1: u8, reg2: u8 },           // 8XYE
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
    StoreMem { to_reg: u8 },                    // FX55
    LoadMem { to_reg: u8 },                     // FX65
}

impl Instruction {
    pub fn decode(instr: u16) -> Instruction {
        let opcode = (instr & 0xF000) >> 12;

        use Instruction::*;
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
                0x6 => ShiftRight {
                    reg1: d_reg1(instr),
                    reg2: d_reg2(instr),
                },
                0x7 => Sub {
                    reg1: d_reg2(instr),
                    reg2: d_reg1(instr),
                },
                0xE => ShiftLeft {
                    reg1: d_reg1(instr),
                    reg2: d_reg2(instr),
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
                0x55 => StoreMem {
                    to_reg: d_reg1(instr),
                },
                0x65 => LoadMem {
                    to_reg: d_reg1(instr),
                },
                _ => Unknown(instr),
            },
            _ => Unknown(instr),
        }
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

macro_rules! decode_tests {
    ( $($label:ident : $inp:expr, $pat:pat,)* ) => {
    $(
        #[test]
        fn $label() {
            assert!(matches!(Instruction::decode($inp), $pat), "got {:?}", Instruction::decode($inp))
        }
    )*
    }
}

decode_tests! {
    t1:  0x00E0, Instruction::ClearScreen,
    t2:  0x00EE, Instruction::ExitSubroutine,
    t3:  0x1EAF, Instruction::Jump { addr: 0x0EAF },
    t4:  0x2FEA, Instruction::CallSubroutine{ addr: 0x0FEA},
    t5:  0x324B, Instruction::SkipValEqual{reg: 0x2, val: 0x4B},
    t6:  0x4401, Instruction::SkipValNotEqual{reg: 0x4, val: 0x01},
    t7:  0x5230, Instruction::SkipRegEqual{reg1: 0x2, reg2: 0x3},
    t8:  0x62F4, Instruction::SetVal{ reg: 2, val: 0xF4},
    t9:  0x713F, Instruction::AddVal{ reg: 1, val: 0x3F},
    t10: 0x8240, Instruction::SetReg{ reg1: 2, reg2: 4},
    t11: 0x8231, Instruction::OR{ reg1: 2, reg2: 3},
    t12: 0x8232, Instruction::AND{ reg1: 2, reg2: 3},
    t13: 0x8233, Instruction::XOR{ reg1: 2, reg2: 3},
    t14: 0x8234, Instruction::Add{ reg1: 2, reg2: 3},
    t15: 0x8235, Instruction::Sub{ reg1: 2, reg2: 3},
    t16: 0x8236, Instruction::ShiftRight{ reg1: 2, reg2: 3},
    t17: 0x9230, Instruction::SkipRegNotEqual{ reg1: 2, reg2: 3},
    t18: 0xA123, Instruction::SetIndex{ val: 0x0123},
    t19: 0xB456, Instruction::JumpOffset{ val: 0x0456},
    t20: 0xC3A5, Instruction::Random{ reg: 3, val: 0xA5},
    t21: 0xD125, Instruction::Display{ reg1: 1, reg2: 2, height: 0x5},
    t22: 0xE19E, Instruction::SkipIfPressed{ reg: 1},
    t23: 0xE1A1, Instruction::SkipNotPressed{ reg: 1},
    t24: 0xF107, Instruction::GetDelayTimer{ reg: 1},
    t25: 0xF215, Instruction::SetDelayTimer{ reg: 2},
    t26: 0xF318, Instruction::SetSoundTimer{ reg: 3},
    t27: 0xF41E, Instruction::AddToIndex{ reg: 4},
    t28: 0xF50A, Instruction::GetKey{ reg: 5},
    t29: 0xF629, Instruction::FontChar{ reg: 6},
    t30: 0xF733, Instruction::BinDecConv{ reg: 7},
    t31: 0xF855, Instruction::StoreMem{ to_reg: 8},
    t32: 0xF965, Instruction::LoadMem{ to_reg: 9},
}

type Addr = u16;
type Vx = u8;
type Vy = u8;
type NN = u8;

#[derive(Debug)]
pub enum Instruction {
    Unknown(u16),
    ClearScreen,             // 00E0
    ExitSubroutine,          // 00EE
    Jump(Addr),              // 1NNN
    CallSubroutine(Addr),    // 2NNN
    SkipValEqual(Vx, NN),    // 3XNN
    SkipValNotEqual(Vx, NN), // 4XNN
    SkipRegEqual(Vx, Vy),    // 5XY0
    SetVal(Vx, NN),          // 6XNN
    AddVal(Vx, NN),          // 7XNN
    SetReg(Vx, Vy),          // 8XY0
    OR(Vx, Vy),              // 8XY1
    AND(Vx, Vy),             // 8XY2
    XOR(Vx, Vy),             // 8XY3
    Add(Vx, Vy),             // 8XY4
    Sub(Vx, Vy),             // 8XY5, 8XY7
    ShiftRight(Vx, Vy),      // 8XY6
    ShiftLeft(Vx, Vy),       // 8XYE
    SkipRegNotEqual(Vx, Vy), // 9XY0
    SetIndex(Addr),          // ANNN
    JumpOffset(Addr),        // BNNN
    Random(Vx, NN),          // CXNN
    Display(Vx, Vy, u8),     // DXYN
    SkipIfPressed(Vx),       // EX9E
    SkipNotPressed(Vx),      // EXA1
    GetDelayTimer(Vx),       // FX07
    SetDelayTimer(Vx),       // FX15
    SetSoundTimer(Vx),       // FX18
    AddToIndex(Vx),          // FX1E
    GetKey(Vx),              // FX0A
    FontChar(Vx),            // FX29
    BinDecConv(Vx),          // FX33
    StoreMem(Vx),            // FX55
    LoadMem(Vx),             // FX65
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
            0x1 => Jump(instr & 0x0FFF),
            0x2 => CallSubroutine(instr & 0x0FFF),
            0x3 => SkipValEqual(d_reg1(instr), d_val(instr)),
            0x4 => SkipValNotEqual(d_reg1(instr), d_val(instr)),
            0x5 => SkipRegEqual(d_reg1(instr), d_reg2(instr)),
            0x6 => SetVal(d_reg1(instr), d_val(instr)),
            0x7 => AddVal(d_reg1(instr), d_val(instr)),
            0x8 => match instr & 0x000F {
                0x0 => SetReg(d_reg1(instr), d_reg2(instr)),
                0x1 => OR(d_reg1(instr), d_reg2(instr)),
                0x2 => AND(d_reg1(instr), d_reg2(instr)),
                0x3 => XOR(d_reg1(instr), d_reg2(instr)),
                0x4 => Add(d_reg1(instr), d_reg2(instr)),
                0x5 => Sub(d_reg1(instr), d_reg2(instr)),
                0x6 => ShiftRight(d_reg1(instr), d_reg2(instr)),
                0x7 => Sub(d_reg2(instr), d_reg1(instr)),
                0xE => ShiftLeft(d_reg1(instr), d_reg2(instr)),
                _ => Unknown(instr),
            },
            0x9 => SkipRegNotEqual(d_reg1(instr), d_reg2(instr)),
            0xA => SetIndex(instr & 0x0FFF),
            0xB => JumpOffset(instr & 0x0FFF),
            0xC => Random(d_reg1(instr), d_val(instr)),
            0xD => Display(d_reg1(instr), d_reg2(instr), (instr & 0x000F) as u8),
            0xE => match instr & 0x00FF {
                0x9E => SkipIfPressed(d_reg1(instr)),
                0xA1 => SkipNotPressed(d_reg1(instr)),
                _ => Unknown(instr),
            },
            0xF => match instr & 0x00FF {
                0x07 => GetDelayTimer(d_reg1(instr)),
                0x15 => SetDelayTimer(d_reg1(instr)),
                0x18 => SetSoundTimer(d_reg1(instr)),
                0x1E => AddToIndex(d_reg1(instr)),
                0x0A => GetKey(d_reg1(instr)),
                0x29 => FontChar(d_reg1(instr)),
                0x33 => BinDecConv(d_reg1(instr)),
                0x55 => StoreMem(d_reg1(instr)),
                0x65 => LoadMem(d_reg1(instr)),
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
    t3:  0x1EAF, Instruction::Jump(0x0EAF),
    t4:  0x2FEA, Instruction::CallSubroutine(0x0FEA),
    t5:  0x324B, Instruction::SkipValEqual(0x2, 0x4B),
    t6:  0x4401, Instruction::SkipValNotEqual(0x4, 0x01),
    t7:  0x5230, Instruction::SkipRegEqual(0x2, 0x3),
    t8:  0x62F4, Instruction::SetVal(2, 0xF4),
    t9:  0x713F, Instruction::AddVal(1, 0x3F),
    t10: 0x8240, Instruction::SetReg(2, 4),
    t11: 0x8231, Instruction::OR(2, 3),
    t12: 0x8232, Instruction::AND(2, 3),
    t13: 0x8233, Instruction::XOR(2, 3),
    t14: 0x8234, Instruction::Add(2, 3),
    t15: 0x8235, Instruction::Sub(2, 3),
    t16: 0x8236, Instruction::ShiftRight(2, 3),
    t17: 0x9230, Instruction::SkipRegNotEqual(2, 3),
    t18: 0xA123, Instruction::SetIndex(0x0123),
    t19: 0xB456, Instruction::JumpOffset(0x0456),
    t20: 0xC3A5, Instruction::Random(3, 0xA5),
    t21: 0xD125, Instruction::Display(1, 2, 0x5),
    t22: 0xE19E, Instruction::SkipIfPressed(1),
    t23: 0xE1A1, Instruction::SkipNotPressed(1),
    t24: 0xF107, Instruction::GetDelayTimer(1),
    t25: 0xF215, Instruction::SetDelayTimer(2),
    t26: 0xF318, Instruction::SetSoundTimer(3),
    t27: 0xF41E, Instruction::AddToIndex(4),
    t28: 0xF50A, Instruction::GetKey(5),
    t29: 0xF629, Instruction::FontChar(6),
    t30: 0xF733, Instruction::BinDecConv(7),
    t31: 0xF855, Instruction::StoreMem(8),
    t32: 0xF965, Instruction::LoadMem(9),
}

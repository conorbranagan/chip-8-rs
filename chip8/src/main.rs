// Sound & Display

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

struct Display {
    pixels: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]
}

impl Display {
    fn new() -> Display {
        Display{
            pixels: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT]
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
    [0xF0, 0x80, 0xF0, 0x80, 0x80],  // F
];

static DELAY_TIMER: u8 = 0;
static SOUND_TIMER: u8 = 0;

// Memory

const RAM_SIZE: usize = 4 * 1024;

struct Memory {
    data: [u8; RAM_SIZE],
    pc: u32 // should this be separate?
}

impl Memory {
    fn new() -> Memory {
        Memory{
            data: [0; RAM_SIZE],
            pc: 0,
        }
    }
}

const NUM_REGISTERS: usize = 16;

struct IndexRegister {
    val: u8
}

impl IndexRegister {
    fn new() -> IndexRegister {
        IndexRegister{val: 0}
    }

    fn set(&mut self, val: u8) {
        self.val = val;
    }
}

struct Registers {
    // Registers are V0 through VF
    data: [u8; NUM_REGISTERS]
}

impl Registers {
    fn new() -> Registers {
        return Registers{
            data: [0; NUM_REGISTERS]
        }
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
    max_size: usize
}

impl Stack {
    fn new(max_size: usize) -> Stack {
        Stack{
            data: Vec::with_capacity(max_size),
            sp: 0,
            max_size
        }
    }

    fn push(&mut self, value: u16) {
        if self.sp >= self.max_size {
            // stack overflow!
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
        }
        self.sp -= 1;
        self.data[self.sp]
    }

    // TODO: push, pop, peek, clear?
}

fn main() {
    let memory = Memory::new();
    let display = Display::new();
    let stack = Stack::new(16);
    let index_register = IndexRegister::new();

    loop {
        // fetch()
        // decode()
        // execute()
    }
}

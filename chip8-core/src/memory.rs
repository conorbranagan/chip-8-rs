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

pub(crate) struct Memory {
    data: [u8; RAM_SIZE],
}

impl Memory {
    pub(crate) fn new() -> Memory {
        let mut m = Memory {
            data: [0; RAM_SIZE],
        };
        for (i, row) in FONT.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                m.data[(i * row.len()) * j] = *col
            }
        }
        m
    }

    pub(crate) fn write(&mut self, addr: usize, val: u8) {
        self.data[addr] = val;
    }

    pub(crate) fn read(&mut self, addr: usize) -> u8 {
        self.data[addr]
    }
}

static MAX_STACK_SIZE: usize = 100;

pub struct Stack {
    data: Vec<u16>,
    sp: usize,
    max_size: usize,
}

impl Stack {
    pub(crate) fn default() -> Stack {
        Stack::new(MAX_STACK_SIZE)
    }

    pub(crate) fn new(max_size: usize) -> Stack {
        Stack {
            data: Vec::with_capacity(max_size),
            sp: 0,
            max_size,
        }
    }

    pub(crate) fn push(&mut self, value: u16) {
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

    pub(crate) fn pop(&mut self) -> u16 {
        if self.sp == 0 {
            // stack underflow!
            panic!("stack underflow")
        }
        self.sp -= 1;
        self.data[self.sp]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory() {
        let mut memory = Memory::new();
        memory.write(0x12, 1);
        assert_eq!(memory.read(0x12), 1);
        assert_eq!(memory.read(0x13), 0);
    }

    #[test]
    fn test_stack() {
        let mut stack = Stack::new(MAX_STACK_SIZE);
        stack.push(1);
        assert_eq!(stack.pop(), 1);
    }
}

use crate::vm::VMError;

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

pub(crate) const RAM_SIZE: usize = 4 * 1024;

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
                let idx = i * row.len() + j;
                m.data[idx] = *col;
            }
        }
        m
    }

    pub(crate) fn write(&mut self, addr: usize, val: u8) -> Result<(), VMError> {
        if addr >= RAM_SIZE {
            return Err(VMError::MemoryOutOfBounds(addr));
        }
        self.data[addr] = val;
        Ok(())
    }

    pub(crate) fn read(&self, addr: usize) -> Result<u8, VMError> {
        if addr >= RAM_SIZE {
            return Err(VMError::MemoryOutOfBounds(addr));
        }
        Ok(self.data[addr])
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

    pub(crate) fn push(&mut self, value: u16) -> Result<(), VMError> {
        if self.sp >= self.max_size {
            return Err(VMError::StackOverflow());
        }
        if self.sp == self.data.len() {
            self.data.push(value);
        } else {
            self.data[self.sp] = value;
        }
        self.sp += 1;
        Ok(())
    }

    pub(crate) fn pop(&mut self) -> Result<u16, VMError> {
        if self.sp == 0 {
            return Err(VMError::StackUnderflow());
        }
        self.sp -= 1;
        Ok(self.data[self.sp])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory() {
        let mut memory = Memory::new();
        assert!(memory.write(0x212, 1).is_ok());
        assert_eq!(memory.read(0x212).unwrap(), 1);
        assert_eq!(memory.read(0x213).unwrap(), 0);
        assert_eq!(memory.read(0x0).unwrap(), FONT[0][0]);
    }

    #[test]
    fn test_stack() {
        let mut stack = Stack::new(MAX_STACK_SIZE);
        assert!(stack.push(1).is_ok());
        let result = stack.pop();
        assert_eq!(result.unwrap(), 1);
        assert!(stack.pop().is_err());
    }
}

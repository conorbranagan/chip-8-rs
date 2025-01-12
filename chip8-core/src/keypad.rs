use crate::vm::VMError;
use std::ops::{Index, IndexMut};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    /// Key `0`
    Key0 = 0x0,
    /// Key `1`
    Key1 = 0x1,
    /// Key `2`
    Key2 = 0x2,
    /// Key `3`
    Key3 = 0x3,
    /// Key `4`
    Key4 = 0x4,
    /// Key `5`
    Key5 = 0x5,
    /// Key `6`
    Key6 = 0x6,
    /// Key `7`
    Key7 = 0x7,
    /// Key `8`
    Key8 = 0x8,
    /// Key `9`
    Key9 = 0x9,
    /// Key `A`
    KeyA = 0xA,
    /// Key `B`
    KeyB = 0xB,
    /// Key `C`
    KeyC = 0xC,
    /// Key `D`
    KeyD = 0xD,
    /// Key `E`
    KeyE = 0xE,
    /// Key `F`
    KeyF = 0xF,
}

impl TryFrom<u8> for Key {
    type Error = VMError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Key::*;
        match value {
            0x0 => Ok(Key0),
            0x1 => Ok(Key1),
            0x2 => Ok(Key2),
            0x3 => Ok(Key3),
            0x4 => Ok(Key4),
            0x5 => Ok(Key5),
            0x6 => Ok(Key6),
            0x7 => Ok(Key7),
            0x8 => Ok(Key8),
            0x9 => Ok(Key9),
            0xA => Ok(KeyA),
            0xB => Ok(KeyB),
            0xC => Ok(KeyC),
            0xD => Ok(KeyD),
            0xE => Ok(KeyE),
            0xF => Ok(KeyF),
            _ => Err(VMError::UnknownKey(value)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum KeyWait {
    NotWaiting,
    WaitingForPress(u8),
    WaitingForRelease(u8),
}

impl KeyWait {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    NotPressed,
    Pressed,
}

pub(crate) struct Keypad {
    state: [KeyState; 16],
    wait_state: KeyWait,
}

impl Keypad {
    pub(crate) fn new() -> Self {
        Keypad {
            state: [KeyState::NotPressed; 16],
            wait_state: KeyWait::NotWaiting,
        }
    }

    pub(crate) fn is_waiting(&mut self) -> bool {
        match self.wait_state {
            KeyWait::NotWaiting => false,
            KeyWait::WaitingForPress(_) => true,
            KeyWait::WaitingForRelease(_) => true,
        }
    }

    pub(crate) fn wait_state(&mut self) -> KeyWait {
        self.wait_state
    }

    pub(crate) fn set_wait(&mut self, wait_state: KeyWait) {
        self.wait_state = wait_state
    }
}

impl Index<Key> for Keypad {
    type Output = KeyState;

    fn index(&self, index: Key) -> &Self::Output {
        &self.state[index as usize]
    }
}

impl IndexMut<Key> for Keypad {
    fn index_mut(&mut self, index: Key) -> &mut Self::Output {
        &mut self.state[index as usize]
    }
}

/// Individual key on the [`Keypad`]
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

pub(crate) struct Keypad {}

impl Keypad {
    pub(crate) fn new() -> Self {
        Keypad {}
    }

    pub(crate) fn pressed(&mut self) -> Key {
        Key::Key0
    }
}

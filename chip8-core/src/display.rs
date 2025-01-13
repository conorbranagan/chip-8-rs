pub struct Display {
    pixels: [[bool; Display::WIDTH]; Display::HEIGHT],
}

impl Display {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub(crate) fn new() -> Display {
        Display {
            pixels: [[false; Display::WIDTH]; Display::HEIGHT],
        }
    }

    pub(crate) fn set(&mut self, x: usize, y: usize, val: bool) {
        let wrapped_y = y & (Display::HEIGHT - 1);
        let wrapped_x = x & (Display::WIDTH - 1);
        self.pixels[wrapped_y][wrapped_x] = val;
    }

    pub(crate) fn get(&self, x: usize, y: usize) -> Result<bool, String> {
        let wrapped_y = y & (Display::HEIGHT - 1);
        let wrapped_x = x & (Display::WIDTH - 1);
        Ok(self.pixels[wrapped_y][wrapped_x])
    }

    pub(crate) fn clear(&mut self) {
        self.pixels = [[false; Display::WIDTH]; Display::HEIGHT];
    }

    pub(crate) fn get_framebuffer(&mut self) -> &[bool] {
        self.pixels.as_flattened()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let mut display = Display::new();
        display.set(1, 2, true);
        assert_eq!(display.get(1, 2).unwrap(), true);
        assert_eq!(display.get(1, 3).unwrap(), false);
        display.clear();
        assert_eq!(display.get(1, 2).unwrap(), false);
    }
}

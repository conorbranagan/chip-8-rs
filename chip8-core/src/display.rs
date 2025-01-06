pub(crate) struct Display {
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
        if y >= self.pixels.len() || x >= self.pixels[y].len() {
            // Cut off bytes outside of the display.
            return;
        }
        self.pixels[y & (Display::HEIGHT - 1) as usize][x & (Display::WIDTH - 1) as usize] = val;
    }

    pub(crate) fn get(&mut self, x: usize, y: usize) -> Result<bool, &str> {
        if y >= self.pixels.len() || x >= self.pixels[y].len() {
            return Err("pixel out of range");
        }
        Ok(self.pixels[y][x])
    }

    pub(crate) fn clear(&mut self) {
        self.pixels = [[false; Display::WIDTH]; Display::HEIGHT];
    }

    pub(crate) fn render(&mut self) {
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

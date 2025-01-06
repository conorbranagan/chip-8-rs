const DISPLAY_WIDTH: u8 = 64;
const DISPLAY_HEIGHT: u8 = 32;

pub(crate) fn new() -> Display {
    Display::new()
}

pub(crate) struct Display {
    pixels: [[bool; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
}

impl Display {
    pub(crate) fn new() -> Display {
        Display {
            pixels: [[false; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
        }
    }

    pub(crate) fn set(&mut self, x: usize, y: usize, val: bool) {
        if y >= self.pixels.len() || x >= self.pixels[y].len() {
            // Cut off bytes outside of the display.
            return;
        }
        self.pixels[y & (DISPLAY_HEIGHT - 1) as usize][x & (DISPLAY_WIDTH - 1) as usize] = val;
    }

    pub(crate) fn get(&mut self, x: usize, y: usize) -> Result<bool, &str> {
        if y >= self.pixels.len() || x >= self.pixels[y].len() {
            return Err("pixel out of range");
        }
        Ok(self.pixels[y][x])
    }

    pub(crate) fn clear(&mut self) {
        self.pixels = [[false; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize];
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

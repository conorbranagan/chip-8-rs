use std::env;

use chip8_core::vm::Chip8VM;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: chip8 <path/to/rom.ch8>");
        return;
    }

    let mut chip8 = Chip8VM::new();
    let rom_path = args.get(1).unwrap();
    chip8.load_rom(rom_path);
    println!("Loaded {} into memory", rom_path);

    let max_instr = 350;
    let mut c = 0;
    loop {
        chip8.execute_next().unwrap();
        c += 1;
        if c >= max_instr {
            break;
        }
    }
}

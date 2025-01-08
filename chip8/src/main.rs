use std::{env, sync::Arc};

use chip8_core::vm::Chip8VM;
use tokio::sync::{mpsc, Mutex};
use tokio::task;

const HZ: u64 = 60; // Equivalent to 5 minutes

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: chip8 <path/to/rom.ch8>");
        return;
    }

    let rom_path = args.get(1).unwrap();
    let mut emu = Emulator::new(rom_path.to_string()).await;

    for _ in 0..350 {
        emu.run_cycle().await;
    }
    emu.shutdown().await;
}

enum EmulatorMessage {
    UpdateDisplay,
    Shutdown,
}

struct Emulator {
    vm: Arc<Mutex<Chip8VM>>,
    display_tx: mpsc::Sender<EmulatorMessage>,
    cycles: Arc<Mutex<u64>>,
}

impl Emulator {
    async fn new(rom_path: String) -> Emulator {
        let vm = Arc::new(Mutex::new(Chip8VM::new()));
        vm.lock().await.load_rom(&rom_path);
        println!("Loaded {} into memory", rom_path);

        let (display_tx, mut display_rx) = mpsc::channel::<EmulatorMessage>(10);
        let display_vm = Arc::clone(&vm);
        task::spawn(async move {
            while let Some(msg) = display_rx.recv().await {
                match msg {
                    EmulatorMessage::UpdateDisplay => {
                        let mut vm = display_vm.lock().await;
                        vm.update_frame();
                    }
                    EmulatorMessage::Shutdown => {
                        break;
                    }
                }
            }
        });

        Emulator {
            vm: vm,
            display_tx,
            cycles: Arc::new(Mutex::new(0)),
        }
    }

    async fn run_cycle(&mut self) {
        let mut vm = self.vm.lock().await;
        vm.execute_next().unwrap();

        let mut cycles = self.cycles.lock().await;
        *cycles += 1;
        if *cycles % HZ == 0 {
            let _ = self.display_tx.try_send(EmulatorMessage::UpdateDisplay);
        }
    }

    async fn shutdown(&mut self) {
        let _ = self.display_tx.try_send(EmulatorMessage::Shutdown);
    }
}

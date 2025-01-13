use chip8_core::display::Display;
use chip8_core::vm::{Chip8VM, VMError};
use pixels::{Pixels, SurfaceTexture};
use simplelog;
use std::fs::File;
use std::path::Path;
use std::time::{Duration, Instant};
use std::{env, sync::Arc};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 256;
const TIMER_INTERVAL: Duration = Duration::from_micros(1_000_000 / 60); // 60Hz
const CYCLE_INTERVAL: Duration = Duration::from_micros(1_000_000 / 500); // 500Hz
const LOG_FILE: &str = "chip8-debug.log";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: chip8 <path/to/rom.ch8>");
        return;
    }

    let log_file = File::create(LOG_FILE).unwrap();
    simplelog::CombinedLogger::init(vec![simplelog::WriteLogger::new(
        // Set to debug to get full instruction logging.
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        log_file,
    )])
    .unwrap();

    let rom_path = args.get(1).unwrap();
    match Emulator::new(rom_path.to_string()) {
        Ok(mut emu) => {
            let event_loop: EventLoop<()> = EventLoop::new().unwrap();
            event_loop.set_control_flow(ControlFlow::Poll);
            let _ = event_loop.run_app(&mut emu);
        }
        Err(e) => {
            println!("Failed to start emulator: {}", e)
        }
    }
}

struct Emulator {
    vm: Chip8VM,
    rom_name: String,
    window: Option<Arc<Window>>,
    frame_buffer: Option<Pixels<'static>>,
    // manage cycle and timer iterations independently
    last_cycle: Instant,
    last_timer_update: Instant,
}

impl Emulator {
    fn new(rom_path: String) -> Result<Self, VMError> {
        let mut vm = Chip8VM::new();
        vm.load_rom(&rom_path)?;
        let file_name = Path::new(rom_path.as_str())
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        Ok(Self {
            vm: vm,
            rom_name: file_name,
            window: None,
            frame_buffer: None,
            last_cycle: Instant::now(),
            last_timer_update: Instant::now(),
        })
    }

    fn cycle(&mut self) -> Result<(), VMError> {
        let now = Instant::now();
        if now.duration_since(self.last_cycle) > CYCLE_INTERVAL {
            self.vm.cycle()?;
            self.last_cycle = now;
        }

        if now.duration_since(self.last_timer_update) > TIMER_INTERVAL {
            self.vm.tick_timers();
            self.last_timer_update = now;

            // Redraw at the timer frequency of 60hz
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }

        Ok(())
    }

    fn draw_frame(&mut self) {
        if let Some(pixels) = &mut self.frame_buffer {
            let vm_frame = self.vm.get_frame_buffer();

            // Each pixel is 4 bytes (rbga) so we chunk and map from bool buf -> pixels.
            for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                let vm_pixel = vm_frame[i];
                // purple on black background
                let rgba = if vm_pixel {
                    [0x5e, 0x48, 0xe8, 0xff]
                } else {
                    [0x0, 0x0, 0x0, 0xff]
                };
                pixel.copy_from_slice(&rgba);
            }
            pixels.render().unwrap();
        }
    }

    fn handle_key(&mut self, code: KeyCode, is_pressed: bool) {
        // Map key codes to computer-keyboard-friendly codes.
        // [1, 2, 3, 4]
        // [Q, W, E, R]
        // [A, S, D, F]
        // [Z, X, C, V]
        let key_code: u8 = {
            match code {
                KeyCode::Digit1 => 0x1,
                KeyCode::Digit2 => 0x2,
                KeyCode::Digit3 => 0x3,
                KeyCode::Digit4 => 0xC,
                KeyCode::KeyQ => 0x4,
                KeyCode::KeyW => 0x5,
                KeyCode::KeyE => 0x6,
                KeyCode::KeyR => 0xD,
                KeyCode::KeyA => 0x7,
                KeyCode::KeyS => 0x8,
                KeyCode::KeyD => 0x9,
                KeyCode::KeyF => 0xE,
                KeyCode::KeyZ => 0xA,
                KeyCode::KeyX => 0x0,
                KeyCode::KeyC => 0xB,
                KeyCode::KeyV => 0xF,
                _ => 0x10,
            }
        };
        self.vm.handle_key(key_code, is_pressed);
    }
}

impl ApplicationHandler for Emulator {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title(format!("Chip-8 - {}", self.rom_name))
            .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .with_min_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let fb = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, window.clone());
            Pixels::new(
                Display::WIDTH as u32,
                Display::HEIGHT as u32,
                surface_texture,
            )
            .unwrap()
        };

        self.window = Some(window.clone());
        self.frame_buffer = Some(fb);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(pixels) = &mut self.frame_buffer {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        println!("pixels.resize_surface: {:?}", err);
                        event_loop.exit();
                        return;
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.draw_frame();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    self.handle_key(code, event.state.is_pressed());
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Err(err) = self.cycle() {
            println!("failed to run cycle: {}", err);
            event_loop.exit();
        }
    }
}

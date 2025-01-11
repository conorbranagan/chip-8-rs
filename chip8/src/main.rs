use std::{env, sync::Arc};

use chip8_core::display::Display;
use chip8_core::vm::{Chip8VM, VMError};
use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

const HZ: u64 = 60;
const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 256;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: chip8 <path/to/rom.ch8>");
        return;
    }

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
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    vm: Chip8VM,
    cycles: u64,
}

impl Emulator {
    fn new(rom_path: String) -> Result<Self, VMError> {
        let mut vm = Chip8VM::new()?;
        vm.load_rom(&rom_path)?;
        Ok(Self {
            pixels: None,
            vm: vm,
            cycles: 0,
            window: None,
        })
    }

    fn run_cycle(&mut self) -> Result<(), VMError> {
        self.vm.run_cycle()?;
        self.cycles += 1;
        if self.cycles % HZ == 0 {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
        Ok(())
    }

    fn draw_frame(&mut self) {
        if let Some(pixels) = &mut self.pixels {
            let vm_frame = &self.vm.get_framebuffer();

            // Each pixel is 4 bytes (rbga) so we chunk and map from bool buf -> pixels.
            for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                let vm_pixel = vm_frame[i];
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
        let key_code: u8 = {
            match code {
                KeyCode::Numpad1 => 0x0,
                KeyCode::Numpad2 => 0x1,
                KeyCode::Numpad3 => 0x2,
                KeyCode::Numpad4 => 0x3,
                KeyCode::KeyQ => 0x4,
                KeyCode::KeyW => 0x5,
                KeyCode::KeyE => 0x6,
                KeyCode::KeyR => 0x7,
                KeyCode::KeyA => 0x8,
                KeyCode::KeyS => 0x9,
                KeyCode::KeyD => 0xA,
                KeyCode::KeyF => 0xB,
                KeyCode::KeyZ => 0xC,
                KeyCode::KeyX => 0xD,
                KeyCode::KeyC => 0xE,
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
            .with_title("Chip-8 Emulator")
            .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .with_min_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let pixels = {
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
        self.pixels = Some(pixels);
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
                if let Some(pixels) = &mut self.pixels {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        println!("pixels.resize_surface: {:?}", err);
                        event_loop.exit();
                        return;
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.draw_frame();
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
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
        if let Err(err) = self.run_cycle() {
            println!("failed to run cycle: {}", err);
            event_loop.exit();
        }
    }
}

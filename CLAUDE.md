# CHIP-8 Emulator - Codebase Documentation

## Project Overview

This is a CHIP-8 emulator written in Rust, implementing the classic CHIP-8 virtual machine specification. The project demonstrates clean architecture principles with a clear separation between the emulation core and the GUI frontend. It consists of approximately 1,130 lines of Rust code organized into two crates within a Cargo workspace.

**Technology Stack**: Rust (Edition 2021), cross-platform (Linux, macOS, Windows)

## Project Structure

```
/root/repo/
├── chip8-core/           # Core VM implementation (library crate)
│   └── src/
│       ├── lib.rs        # Public module exports
│       ├── vm.rs         # Main VM engine (461 lines)
│       ├── instructions.rs # Instruction decoder (159 lines)
│       ├── memory.rs     # Memory management (113 lines)
│       ├── display.rs    # Display/frame buffer (56 lines)
│       └── keypad.rs     # Keypad input handling (125 lines)
├── chip8/                # GUI application (binary crate)
│   └── src/
│       └── main.rs       # Window/rendering/event loop (211 lines)
├── bin/
│   └── test-roms/        # Test ROM files for validation
├── images/               # Screenshot documentation
├── README.md             # User-facing documentation
└── Cargo.toml            # Workspace configuration
```

## Architecture

### Multi-Layer Modular Design

```
┌─────────────────────────────────────┐
│   GUI Application Layer             │
│   (chip8/src/main.rs)               │
│   - Window management (winit)       │
│   - GPU rendering (pixels)          │
│   - Input handling                  │
│   - Frame rendering loop            │
└────────────┬────────────────────────┘
             │
     ┌───────▼────────────────────────┐
     │  VM Abstraction Layer          │
     │  (chip8-core/src/vm.rs)        │
     │  - Instruction cycle control   │
     │  - State management            │
     │  - Error handling              │
     │  - Timer management            │
     └───────┬────────────────────────┘
             │
     ┌───────▼────────────────────────┐
     │  Component Modules             │
     │  ├─ instructions.rs (decoder)  │
     │  ├─ memory.rs (RAM/stack)      │
     │  ├─ display.rs (frame buffer)  │
     │  └─ keypad.rs (input)          │
     └────────────────────────────────┘
```

### Design Principles

1. **Separation of Concerns**: VM logic is completely isolated from rendering and I/O operations
2. **Error Handling**: Custom error types with context-specific messages using `thiserror`
3. **Modular Components**: Each system (memory, display, keypad) is encapsulated in its own module
4. **Independent Timing**: CPU cycles (500Hz) and timers (60Hz) run asynchronously
5. **State Machine Pattern**: Keyboard input uses explicit wait states for blocking operations

## Core Components

### chip8-core Crate

#### vm.rs (461 lines)
The central controller implementing the CHIP-8 virtual machine:

- **Chip8VM struct**: Contains all VM state
  - Memory, Display, Registers (V0-VF), Stack
  - Program Counter (PC), Index Register (I)
  - Delay Timer and Sound Timer
  - Keypad state manager

- **Key Methods**:
  - `new()`: Initialize VM with all components, pre-load font data
  - `load_rom()`: Load binary ROM files into memory starting at 0x200
  - `cycle()`: Execute single fetch-decode-execute cycle
  - `execute()`: Dispatch and execute 35+ CHIP-8 instruction types
  - `tick_timers()`: Decrement timers at 60Hz rate
  - `handle_key()`: Process keyboard input with key-wait state management

- **Error Handling**: Custom `VMError` enum with variants:
  - UnknownInstruction
  - StackOverflow/StackUnderflow
  - ROMLoadError
  - InvalidMemoryAccess

#### instructions.rs (159 lines)
Instruction decoding and representation:

- **Instruction enum**: 35 CHIP-8 instruction variants
  - Control flow: Jump, CallSubroutine, ExitSubroutine, SkipIf*
  - Arithmetic: Add, SubLeft, SubRight, AND, OR, XOR, ShiftLeft, ShiftRight
  - Display: Display (sprite drawing with collision detection)
  - Memory: LoadI, StoreMem, LoadMem, BinDecConv
  - I/O: Keypad input, GetDelayTimer, SetDelayTimer, SetSoundTimer

- **decode() method**: Converts 16-bit opcodes to Instruction enums
- **Comprehensive test suite**: 32 unit tests for opcode decoding

#### memory.rs (113 lines)
Memory and stack management:

- **Memory struct**: 4KB RAM array (4096 bytes)
  - ROM loads at address 0x200
  - Font data (16 characters) pre-initialized at memory start
  - Read/write operations with bounds checking

- **Stack struct**: Vector-based call stack
  - Configurable maximum size (default: 100)
  - Push/pop operations with overflow/underflow detection

#### display.rs (56 lines)
Graphics frame buffer:

- **Display struct**: 64x32 pixel monochrome display
  - 2D boolean array for pixel states (true = on, false = off)
  - Wrapping coordinates (toroidal display topology)
  - XOR-based pixel drawing for collision detection

- **Methods**: `set()`, `get()`, `clear()`, `get_frame_buffer()`

#### keypad.rs (125 lines)
Input handling with state machine:

- **Key enum**: 16 hexadecimal keys (0x0 to 0xF)
- **KeyState**: Pressed/NotPressed tracking
- **KeyWait**: State machine for blocking input
  - WaitingForPress: Pause execution until key press
  - WaitingForRelease: Wait for key release confirmation
  - NotWaiting: Normal execution

- **Keypad struct**: Manages 16-key state array with wait state logic

### chip8 Crate

#### main.rs (211 lines)
GUI application implementing the emulator frontend:

- **Emulator struct**: Main application state
  - Embeds Chip8VM instance
  - Manages window lifecycle and frame buffer
  - Tracks timing for independent cycle and timer intervals

- **Window Configuration**:
  - Display size: 512x256 pixels (8x multiplier of 64x32 VM display)
  - Color scheme: Purple (#5e48e8) on black background
  - Uses `winit` for cross-platform windowing
  - Uses `pixels` for GPU-accelerated rendering

- **Timing Model**:
  - **Cycle interval**: 500Hz (every 2ms) - instruction execution
  - **Timer interval**: 60Hz (every ~16.67ms) - display refresh and timer decrements
  - Independent intervals allow accurate timing

- **Keyboard Mapping**: QWERTY to hexadecimal layout
  ```
  [1, 2, 3, 4]  →  [0x1, 0x2, 0x3, 0xC]
  [Q, W, E, R]  →  [0x4, 0x5, 0x6, 0xD]
  [A, S, D, F]  →  [0x7, 0x8, 0x9, 0xE]
  [Z, X, C, V]  →  [0xA, 0x0, 0xB, 0xF]
  ```

- **Event Loop**: Handles window events (close, keyboard, resize, redraw)

## Dependencies

### chip8-core Dependencies
```toml
rand = "0.8"           # Random number generation (CXNN instruction)
log = "0.4"            # Logging framework for instruction tracing
thiserror = "2.0.11"   # Error type derivation macros
```

### chip8 Application Dependencies
```toml
chip8_core = { path = "../chip8-core" }  # Local core VM crate
pixels = "0.15.0"                         # GPU rendering abstraction
winit = "0.30.8"                          # Cross-platform windowing
winit_input_helper = "0.15"               # Input event helpers
log = "0.4"                               # Logging framework
simplelog = "0.12"                        # Console/file logging implementation
```

## CHIP-8 Implementation Details

### Specifications
- **Registers**: 16 general-purpose 8-bit registers (V0-VF), 16-bit Index register (I)
- **Memory**: 4KB RAM (0x000-0xFFF), ROM starts at 0x200
- **Display**: 64x32 monochrome pixels with XOR-based sprite drawing
- **Stack**: Subroutine call stack with overflow protection
- **Timers**: Delay timer and sound timer (both decrement at 60Hz)
- **Input**: 16-key hexadecimal keypad with blocking GetKey operation
- **Instructions**: 35 opcodes from CHIP-8 specification

### Font Data
Built-in hexadecimal font sprites (0-F) pre-loaded in memory at startup.

### Collision Detection
Display drawing uses XOR logic - VF register set to 1 if any pixel is flipped from on to off.

## Testing

### Unit Tests
- **instructions.rs**: 32 opcode decoding tests
- **memory.rs**: Memory read/write and stack operations
- **display.rs**: Pixel setting, wrapping, and coordinate tests
- **vm.rs**: Register overflow/underflow behavior

### Integration Tests
The emulator has been validated against several test ROMs:
- **Corax+ Opcode Test**: Comprehensive instruction validation
- **Flags Test**: Register flag behavior verification
- **Space Invaders**: Real-world game compatibility
- **Tetris**: Complex game logic testing

Test ROM files are located in `bin/test-roms/`.

## Building and Running

### Build
```bash
# Build debug version
cargo build

# Build optimized release version
cargo build --release
```

### Run
```bash
# Run with a ROM file
cargo run --release -- <path/to/rom.ch8>

# Example
cargo run --release -- bin/test-roms/tetris.ch8
```

### Workspace Structure
The project uses a Cargo workspace with two member crates:
- `chip8-core`: Library crate (VM engine)
- `chip8`: Binary crate (GUI application)

## Known Limitations and TODOs

### Not Yet Implemented
- [ ] Sound/Beep functionality (sound timer decrements but no audio output)
- [ ] Various CHIP-8 quirks and edge cases
- [ ] FontChar (FX29) instruction - partial implementation

### Potential Improvements
- Add configurable CPU speed
- Implement CHIP-8 variants (SUPER-CHIP, XO-CHIP)
- Add debugger/disassembler tools
- Support ROM file drag-and-drop

## Development Notes

- Most instruction implementations based on [Tobias V. Langhoff's guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- Test suite from [Timendus's chip8-test-suite](https://github.com/Timendus/chip8-test-suite)
- First significant Rust project by the author - feedback welcome!

## Key Files Reference

For detailed implementation, refer to:
- VM core logic: `chip8-core/src/vm.rs:1-461`
- Instruction decoding: `chip8-core/src/instructions.rs:1-159`
- Memory management: `chip8-core/src/memory.rs:1-113`
- Display rendering: `chip8-core/src/display.rs:1-56`
- Input handling: `chip8-core/src/keypad.rs:1-125`
- GUI application: `chip8/src/main.rs:1-211`

## Git Repository

- **Current Branch**: terragon/update-claude-md-5it5u2
- **Main Branch**: main
- **Status**: Clean working directory

Recent commits focus on:
- Display improvements and bug fixes
- Timer independence and interval adjustments
- Code quality and naming improvements

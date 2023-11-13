use std::{io, thread, time::Duration};

use device_query::{device_state, DeviceQuery, DeviceState};

fn main() {
    let device_state = DeviceState::new();

    loop {
        let keys = device_state.get_keys();

        println!("{:#?}", keys)
    }

    let mut chip8 = Chip8::new();

    chip8.run();
}

struct Chip8 {
    memory: [u8; 4 * 1024],
    // Display is updated at 60hz
    display: [u8; (64 * 32) / 8],
    stack: [u8; 2 * 16],
    program_counter: u16,
    index_register: u16,
    variable_registers: [u8; 16],
    // Timers decrement by 1 at 60hz, independent of execution speed
    delay_timer: u8,
    sound_timer: u8,
}
impl Chip8 {
    fn new() -> Self {
        let mut memory = [0; 4 * 1024];

        let mut font = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        // todo: make this a constant
        let font_location = 0x50;

        // place the font into memory at the specified address
        memory[font_location..font_location + font.len()].swap_with_slice(&mut font);

        Self {
            memory,
            display: [0; (64 * 32) / 8],
            stack: [0; 2 * 16],
            program_counter: 0,
            index_register: 0,
            variable_registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    fn run(&mut self) {
        loop {
            // fetch
            let instruction = self
                .memory
                .get(self.program_counter as _..=(self.program_counter + 1) as _)
                .expect("Couldn't fetch instruction bytes!");

            // increment program counter
            self.program_counter += 2;

            // decode
            let instruction = InstructionBytes::from_u8s(instruction[0], instruction[1]);
            // let instruction = Instruction

            // execute

            thread::sleep(Duration::from_micros(1_000_000 / 1_000))
        }
    }
}

struct InstructionBytes(u16);
impl InstructionBytes {
    fn from_u16(number: u16) -> Self {
        InstructionBytes(number)
    }
    fn from_u8s(byte1: u8, byte2: u8) -> Self {
        InstructionBytes((byte1 as u16) << 8 | byte2 as u16)
    }

    fn get_opcode(&self) -> u16 {
        (self.0 & 0xF000) >> 12
    }
    fn get_x(&self) -> u16 {
        (self.0 & 0x0F00) >> 8
    }
    fn get_y(&self) -> u16 {
        (self.0 & 0x00F0) >> 4
    }
    fn get_n(&self) -> u16 {
        self.0 & 0x000F
    }
    fn get_nn(&self) -> u16 {
        self.0 & 0x00FF
    }
    fn get_nnn(&self) -> u16 {
        self.0 & 0x0FFF
    }
}

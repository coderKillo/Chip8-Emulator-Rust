use std::usize;

use rand::prelude::*;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const KEYS_SIZE: usize = 16;

const FRONSET_SIZE: usize = 16 * 5;
const FRONTSET: [u8; FRONSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x20, 0x20, 0x20, 0x20, // 1
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

pub struct Emulator {
    pc: u16, // program counter
    ram: [u8; RAM_SIZE],
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; 0xF],
    i_reg: u16,
    sp: u16, // stack pointer
    stack: [u16; STACK_SIZE],
    keys: [bool; KEYS_SIZE],
    dt: u8, // delay timer
    st: u8, // sound timer
}

const START_ADDRESS: u16 = 0x200;

impl Emulator {
    pub fn new() -> Self {
        let mut new_enumlator = Self {
            pc: START_ADDRESS,
            ram: [0; RAM_SIZE],
            display: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; 0xF],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; KEYS_SIZE],
            dt: 0,
            st: 0,
        };

        new_enumlator.ram[..FRONSET_SIZE].copy_from_slice(&FRONTSET);

        new_enumlator
    }

    pub fn get_display(&self) -> &[bool] {
        return &self.display;
    }

    pub fn key_pressed(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDRESS as usize;
        let end = START_ADDRESS as usize + data.len();

        self.ram[start..end].copy_from_slice(data);
    }

    pub fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDRESS;
        self.ram = [0; RAM_SIZE];
        self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; 0xF];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; KEYS_SIZE];
        self.dt = 0;
        self.st = 0;
        self.ram[..FRONSET_SIZE].copy_from_slice(&FRONTSET);
    }

    pub fn tick(&mut self) {
        let operation = self.fetch();

        self.execute(operation);

        // 1. Fetch the value from our game (loaded into RAM) at the memory address stored in our Program Counter.

        // 2. Decode this instruction.

        // 3. Execute, which will possibly involve modifying our CPU registers or RAM.

        // 4. Move the PC to the next instruction and repeat
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
            if self.st == 0 {
                // BEEP
            }
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let mut operation = 0x0000;

        operation |= self.ram[self.pc as usize] as u16;
        operation <<= 8;
        operation |= self.ram[(self.pc + 1) as usize] as u16;

        self.pc += 2;

        operation
    }

    pub fn execute(&mut self, operation: u16) {
        let digit1 = (operation & 0xF000) >> 12;
        let digit2 = (operation & 0x0F00) >> 8;
        let digit3 = (operation & 0x00F0) >> 4;
        let digit4 = (operation & 0x000F) >> 0;

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0) => {
                self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            (0, 0, 0xE, 0xE) => {
                let return_adress = self.pop();
                self.pc = return_adress;
            }
            (1, _, _, _) => {
                let nnn = operation & 0x0FFF;
                self.pc = nnn;
            }
            (2, _, _, _) => {
                let nnn = operation & 0x0FFF;
                self.push(self.pc);
                self.pc = nnn;
            }
            (3, _, _, _) => {
                let nn = (operation & 0x00FF) as u8;
                let x = digit2 as usize;
                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
            }
            (4, _, _, _) => {
                let nn = (operation & 0x00FF) as u8;
                let x = digit2 as usize;
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            }
            (5, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            }
            (6, _, _, _) => {
                let nn = (operation & 0x00FF) as u8;
                let x = digit2 as usize;
                self.v_reg[x] = nn;
            }
            (7, _, _, _) => {
                let nn = (operation & 0x00FF) as u8;
                let x = digit2 as usize;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            }
            (8, _, _, _) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                match digit4 {
                    0 => self.v_reg[x] = self.v_reg[y],
                    1 => self.v_reg[x] |= self.v_reg[y],
                    2 => self.v_reg[x] &= self.v_reg[y],
                    3 => self.v_reg[x] ^= self.v_reg[y],
                    4 => {
                        let (new_value, carry_over) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                        self.v_reg[x] = new_value;
                        self.v_reg[0xF] = if carry_over { 1 } else { 0 };
                    }
                    5 => {
                        let (new_value, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                        self.v_reg[x] = new_value;
                        self.v_reg[0xF] = if borrow { 0 } else { 1 };
                    }
                    6 => {
                        let lsb = self.v_reg[x] & 0x01;
                        self.v_reg[x] >>= 1;
                        self.v_reg[0xF] = lsb;
                    }
                    7 => {
                        let (new_value, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                        self.v_reg[x] = new_value;
                        self.v_reg[0xF] = if borrow { 0 } else { 1 };
                    }
                    0xE => {
                        let msb = (self.v_reg[x] >> 7) & 0x01;
                        self.v_reg[x] <<= 1;
                        self.v_reg[0xF] = msb;
                    }
                    _ => {}
                }
            }
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                let nnn = operation & 0x0FFF;
                self.i_reg = nnn
            }
            (0xB, _, _, _) => {
                let nnn = operation & 0x0FFF;
                self.pc = self.v_reg[0] as u16 + nnn;
            }
            (0xC, _, _, _) => {
                let nn: u8 = operation as u8 & 0xFF;
                let mut rng = rand::thread_rng();
                self.v_reg[digit2 as usize] = rng.gen::<u8>() & nn;
            }
            (0xD, _, _, _) => {
                let x_coord = self.v_reg[digit2 as usize] as u16;
                let y_coord = self.v_reg[digit3 as usize] as u16;
                let height = digit4;
                let mut flipped = false;

                for y_line in 0..height {
                    let pixel = self.ram[(self.i_reg + y_line) as usize];
                    for x_line in 0..8 {
                        let mask = 0b1000_0000 >> x_line;
                        if (mask & pixel) != 0 {
                            let x = (x_line + x_coord) as usize % SCREEN_WIDTH;
                            let y = (y_line + y_coord) as usize % SCREEN_HEIGHT;

                            let idx = x + SCREEN_WIDTH * y;
                            flipped |= self.display[idx];
                            self.display[idx] ^= true;
                        }
                    }
                }

                self.v_reg[0xF] = if flipped { 1 } else { 0 };
            }
            (0xE, _, 9, 0xE) => {
                let x = digit2 as usize;

                if self.keys[self.v_reg[x] as usize] {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 1) => {
                let x = digit2 as usize;

                if !self.keys[self.v_reg[x] as usize] {
                    self.pc += 2;
                }
            }
            (0xF, _, 0, 7) => {
                let x = digit2 as usize;

                self.v_reg[x] = self.dt;
            }
            (0xF, _, 0, 0xA) => {
                let x = digit2 as usize;
                let mut pressed = false;

                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    self.pc -= 2;
                }
            }
            (0xF, _, 1, 5) => {
                let x = digit2 as usize;
                self.dt = self.v_reg[x];
            }
            (0xF, _, 1, 8) => {
                let x = digit2 as usize;
                self.st = self.v_reg[x];
            }
            (0xF, _, 1, 0xE) => {
                let x = digit2 as usize;
                self.i_reg += self.i_reg.wrapping_add(self.v_reg[x] as u16);
            }
            (0xF, _, 2, 9) => {
                let x = digit2 as usize;
                let font_char = self.v_reg[x] as u16;
                self.i_reg = font_char * 5;
            }
            (0xF, _, 3, 3) => {
                let x = digit2 as usize;
                let number = self.v_reg[x];
                let addr = self.i_reg as usize;

                let _100 = number / 100;
                let _10 = (number / 10) % 10;
                let _1 = number % 10;

                self.ram[addr] = _100;
                self.ram[addr + 1] = _10;
                self.ram[addr + 2] = _1;
            }
            (0xF, _, 5, 5) => {
                let x = digit2 as usize;
                for i in 0..=x {
                    self.ram[self.i_reg as usize + i] = self.v_reg[i];
                }
            }
            (0xF, _, 6, 5) => {
                let x = digit2 as usize;
                for i in 0..=x {
                    self.v_reg[i] = self.ram[self.i_reg as usize + i];
                }
            }

            (_, _, _, _) => unimplemented!("operation {} is not implemented", operation),
        }
    }
}

#![no_std]

mod screen;
mod keyboard;
pub mod opcode;

pub use keyboard::Key;
pub use screen::{Window, HEIGHT, WIDTH};
pub use keyboard::Keyboard;


use log::debug;
use rand::{RngCore, SeedableRng, rngs::SmallRng};

use self::screen::VirtualScreen;
use self::opcode::Opcode;

static FONTSET: [u8; 80] =
[
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
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

enum PcInc {
    Inc(u16),
    Val(u16),
    Default,
    None
}

pub trait IO: Keyboard + Window {}

pub struct Chip8<'a, T: Window+Keyboard> {
    // 4096 8bits memory addresses
    memory: [u8; 4096],
    // 16 8bits registers
    register: [u8; 16],
    // 1 16 bits register
    index: u16,
    // Program counter
    pc: u16,
    // Stack containing return addresses
    stack: [u16; 16],
    // Stack pointer
    stack_pointer: usize,

    screen: VirtualScreen,
    io: &'a mut T,
    
    delay_timer: u8,
    sound_timer: u8,

    random: SmallRng,
}



impl<'a, T> Chip8<'a, T> where T: Window + Keyboard {


    pub fn new(io: &'a mut T) -> Chip8<'a, T> {
        Chip8 {
            memory: [0; 4096],
            register: [0; 16],
            index: 0,
            pc: 0x200,
            screen: VirtualScreen::new(),
            stack: [0; 16],
            stack_pointer: 0,
            io: io,
            delay_timer: 0,
            sound_timer: 0,
            random: SmallRng::seed_from_u64(0x649bba8a048482fd),
        }
    }

    

    pub fn init(&mut self, rom: &[u8]) {
        self.memory[..80].copy_from_slice(&FONTSET);
        self.memory[0x200..0x200 + rom.len()].copy_from_slice(&rom);
    }

    pub fn start(&mut self) {
        while self.io.is_running() {
            self.cycle();
            self.display();
        }
    }

    pub fn cycle(&mut self) {
        let hexcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16);
        let opcode = Opcode::decode(hexcode).unwrap();

        debug!("{:#X} {:4X} {}", self.pc, hexcode, opcode);

        let mut pc_inc = PcInc::Default;

        match opcode {
            Opcode::ClearScreen => {
                self.screen.clear();
            },
            Opcode::Return => {
                self.stack_pointer -= 1;
                pc_inc = PcInc::Val(self.stack[self.stack_pointer]);
            },
            Opcode::Jump { addr } => {
                pc_inc = PcInc::Val(addr);
            },
            Opcode::Call { addr } => {
                self.stack[self.stack_pointer] = self.pc + 2;
                self.stack_pointer += 1;
                pc_inc = PcInc::Val(addr);
            },
            Opcode::SkipEqualByte { x, kk } => {
                if self.register[x] == kk { pc_inc = PcInc::Inc(4) }
            },
            Opcode::SkipNotEqualByte { x, kk } => {
                if self.register[x] != kk { pc_inc = PcInc::Inc(4) }
            },
            Opcode::SkipEqual { x, y } => {
                if self.register[x] == self.register[y] { pc_inc = PcInc::Inc(4) }

            },
            Opcode::SkipNotEqual { x, y } => {
                if self.register[x] != self.register[y] { pc_inc = PcInc::Inc(4) }
            },
            Opcode::LoadByte { x, kk } => {
                self.register[x] = kk;
            },
            Opcode::AddByte { x, kk } => {
                self.register[x] = self.register[x].wrapping_add(kk);
            },
            Opcode::Load { x, y } => {
                self.register[x] = self.register[y];
            },
            Opcode::Or { x, y } => {
                self.register[x] |= self.register[y];
            },
            Opcode::And { x, y } => {
                self.register[x] &= self.register[y];
            },
            Opcode::Xor { x, y } => {
                self.register[x] ^= self.register[y];
            },
            Opcode::Add { x, y } => {
                let result = self.register[x].overflowing_add(self.register[y]);
                self.register[x] = result.0;
                if result.1 {
                    self.register[0xF] = 1;
                }
            },
            Opcode::Sub { x, y } => {
                if self.register[x] > self.register[y] {
                    self.register[0xF] = 1;
                }
                else {
                    self.register[0xF] = 0;
                }
                self.register[x] = self.register[x].wrapping_sub(self.register[y]);
            },
            Opcode::Shr { x, y: _ } => {
                self.register[0xF] = self.register[x] & 0x1;
                self.register[x] = self.register[x] / 2;
            },
            Opcode::Shl { x, y: _ } => {
                self.register[0xF] = self.register[x] >> 7 & 0x1;
                self.register[x] = self.register[x].wrapping_mul(2);
            },
            Opcode::Subn { x, y } => {
                if self.register[y] > self.register[x] {
                    self.register[0xF] = 1;
                }
                else {
                    self.register[0xF] = 0;
                }
                self.register[x] = self.register[y] - self.register[x];
            },
            Opcode::LoadI { addr } => {
                self.index = addr;
            },
            Opcode::JumpV0 { addr } => {
                pc_inc = PcInc::Val(addr + self.register[0x0] as u16);
            },
            Opcode::Random { x, kk } => {
                let mut rnd: [u8;1] = [0];
                self.random.fill_bytes(&mut rnd);
                self.register[x] = rnd[0] & kk;
            },
            Opcode::Draw { x, y, n } => {
                let i = self.index as usize;
                self.screen.display(&self.memory[i..(i + n)], self.register[x].into(), self.register[y].into(), &mut self.register[0xF]);
            },
            Opcode::SkipKeyPressed { x } => {
                if self.io.is_key_down(self.register[x].into()) {
                    pc_inc = PcInc::Inc(4);
                }
            },
            Opcode::SkipKeyNotPressed { x } => {
                if !self.io.is_key_down(self.register[x].into()) {
                    pc_inc = PcInc::Inc(4);
                }
            },
            Opcode::LoadDelayTimer { x } => {
                self.register[x] = self.delay_timer;
            },
            Opcode::WaitKeyPressed { x } => {
                pc_inc = PcInc::None;
                self.register[x] = self.io.wait_key_down() as u8;
            },
            Opcode::LoadToDelayTimer { x } => {
                self.delay_timer = self.register[x];
            },
            Opcode::LoadToSoundTimer { x } => {
                self.sound_timer = self.register[x];
            },
            Opcode::AddI { x } => {
                self.index += self.register[x] as u16;
            },
            Opcode::LoadSprite { x } => {
                let addr = (self.register[x]) * 5;
                self.index = addr as u16;
            },
            Opcode::LoadBCD { x } => {
                self.memory[self.index as usize]     = self.register[x] / 100;
                self.memory[(self.index + 1) as usize] = (self.register[x] / 10) % 10;
                self.memory[(self.index + 2) as usize] = (self.register[x] % 100) % 10;
            },
            Opcode::SaveRegisters { x } => {
                for k in 0..=x {
                    self.memory[self.index as usize + k] = self.register[k];
                }
            },
            Opcode::LoadRegisters { x  } => {
                for k in 0..=x {
                    self.register[k] = self.memory[self.index as usize + k];
                }
            },
        }

        match pc_inc {
            PcInc::Val(addr) => self.pc = addr,
            PcInc::Inc(i) => self.pc += i,
            PcInc::Default => self.pc += 2,
            PcInc::None => {}
        }

        if self.delay_timer != 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer != 0 {
            self.sound_timer -= 1;
        }
    }


    pub fn display(&mut self) {
        self.io
        .update_with_buffer(&self.screen.buffer, screen::WIDTH, screen::HEIGHT);
    }
}
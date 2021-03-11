pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub trait Window {
    fn update_with_buffer(&mut self, buffer: &[u8], width: usize, height: usize);
    fn is_running(&mut self) -> bool;
}

pub struct VirtualScreen {
    pub buffer: [u8; WIDTH*HEIGHT],
    pub need_update: bool,
}

impl VirtualScreen {
    pub fn new() -> VirtualScreen {
        VirtualScreen {
            buffer: [0; WIDTH*HEIGHT],
            need_update: true,
        }
    }

    pub fn display(&mut self, sprite: &[u8], x: usize, y: usize, vf: &mut u8) {
        self.need_update = true;
        *vf = 0;
        for row in 0..sprite.len() {
            for col in 0..8usize {
                let pixel = sprite[row as usize] & (0x80 >> col);
                let index = ((y + row) % HEIGHT) * WIDTH + ((x + col) % WIDTH);
                if pixel != 0 {
                    if self.buffer[index] == 1 {
                        *vf = 1;
                    }
                    self.buffer[index] ^= 1;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [0; WIDTH*HEIGHT];
    }
}

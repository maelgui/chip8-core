
#[repr(u8)]
pub enum Key {
    Key0 = 0x0,
    Key1 = 0x1,
    Key2 = 0x2,
    Key3 = 0x3,
    Key4 = 0x4,
    Key5 = 0x5,
    Key6 = 0x6,
    Key7 = 0x7,
    Key8 = 0x8,
    Key9 = 0x9,
    KeyA = 0xA,
    KeyB = 0xB,
    KeyC = 0xC,
    KeyD = 0xD,
    KeyE = 0xE,
    KeyF = 0xF,
}

impl From<u8> for Key {
    fn from(item: u8) -> Self {
        match item {
            0x0 => Key::Key0,
            0x1 => Key::Key1,
            0x2 => Key::Key2,
            0x3 => Key::Key3,
            0x4 => Key::Key4,
            0x5 => Key::Key5,
            0x6 => Key::Key6,
            0x7 => Key::Key7,
            0x8 => Key::Key8,
            0x9 => Key::Key9,
            0xA => Key::KeyA,
            0xB => Key::KeyB,
            0xC => Key::KeyC,
            0xD => Key::KeyD,
            0xE => Key::KeyE,
            0xF => Key::KeyF,
            _ => panic!("Unknown key number")
        }
    }
}

pub trait Keyboard {
    fn is_key_down(&self, key: Key) -> bool;
    fn wait_key_down(&self) -> Key;
}
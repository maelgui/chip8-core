use core::fmt;


pub enum Opcode {
    ClearScreen,
    Return,
    Jump { addr: u16 },
    Call { addr: u16 },
    SkipEqualByte { x: usize, kk: u8 },
    SkipNotEqualByte { x: usize, kk: u8 },
    SkipEqual { x: usize, y: usize },
    SkipNotEqual { x: usize, y: usize },
    LoadByte { x: usize, kk: u8 },
    AddByte { x: usize, kk: u8 },
    Load { x: usize, y: usize },
    Or { x: usize, y: usize },
    And { x: usize, y: usize },
    Xor { x: usize, y: usize },
    Add { x: usize, y: usize },
    Sub { x: usize, y: usize },
    Shr { x: usize, y: usize },
    Shl { x: usize, y: usize },
    Subn { x: usize, y: usize },
    LoadI { addr: u16 },
    JumpV0 { addr: u16 },
    Random { x: usize, kk: u8 },
    Draw { x: usize, y: usize, n: usize },
    SkipKeyPressed { x: usize },
    SkipKeyNotPressed { x: usize },
    LoadDelayTimer { x: usize },
    WaitKeyPressed { x: usize },
    LoadToDelayTimer { x: usize },
    LoadToSoundTimer { x: usize },
    AddI { x: usize },
    LoadSprite { x: usize },
    LoadBCD { x: usize },
    SaveRegisters { x: usize },
    LoadRegisters { x: usize },
}

#[derive(Debug)]
pub enum ParseOpcodeError {
    UnknownOpcode(u16),
}

impl Opcode {
    pub fn decode(opcode: u16) -> Result<Opcode, ParseOpcodeError> {

        let addr = opcode & 0xFFF;
        let x = ((opcode >> 8) & 0xF) as usize;
        let y = ((opcode >> 4) & 0xF) as usize;
        let kk = (opcode & 0xFF) as u8;
        let n = (opcode & 0xF) as usize;

        match opcode >> 12 {
            0x0 => match opcode & 0xFF {
                0xE0 => Ok(Opcode::ClearScreen),
                0xEE => Ok(Opcode::Return),
                _ => Ok(Opcode::Jump { addr }),
            }
            
            0x1 => Ok(Opcode::Jump { addr }),
            0x2 => Ok(Opcode::Call { addr }),
            0x3 => Ok(Opcode::SkipEqualByte { x, kk }),
            0x4 => Ok(Opcode::SkipNotEqualByte { x, kk }),
            0x5 => Ok(Opcode::SkipEqual { x, y }),
            0x6 => Ok(Opcode::LoadByte { x, kk }),
            0x7 => Ok(Opcode::AddByte { x, kk }),
            0x8 => match opcode & 0xF {
                0x0 => Ok(Opcode::Load { x, y }),
                0x1 => Ok(Opcode::Or { x, y }),
                0x2 => Ok(Opcode::And { x, y }),
                0x3 => Ok(Opcode::Xor { x, y }),
                0x4 => Ok(Opcode::Add { x, y }),
                0x5 => Ok(Opcode::Sub { x, y }),
                0x6 => Ok(Opcode::Shr { x, y }),
                0x7 => Ok(Opcode::Subn { x, y }),
                0xE => Ok(Opcode::Shl { x, y }),
                _ => Err(ParseOpcodeError::UnknownOpcode(opcode))
            }
            0x9 => Ok(Opcode::SkipNotEqual { x, y }),
            0xA => Ok(Opcode::LoadI { addr }),
            0xB => Ok(Opcode::JumpV0 { addr }),
            0xC => Ok(Opcode::Random { x, kk }),
            0xD => Ok(Opcode::Draw { x, y, n }),
            0xE => match opcode & 0xFF {
                0x9E => Ok(Opcode::SkipKeyPressed { x }),
                0xA1 => Ok(Opcode::SkipKeyNotPressed { x }),
                _ => Err(ParseOpcodeError::UnknownOpcode(opcode))
            }
            0xF => match opcode & 0xFF {
                0x07 => Ok(Opcode::LoadDelayTimer { x }),
                0x0A => Ok(Opcode::WaitKeyPressed { x }),
                0x15 => Ok(Opcode::LoadToDelayTimer { x }),
                0x18 => Ok(Opcode::LoadToSoundTimer { x }),
                0x1E => Ok(Opcode::AddI { x }),
                0x29 => Ok(Opcode::LoadSprite { x }),
                0x33 => Ok(Opcode::LoadBCD { x }),
                0x55 => Ok(Opcode::SaveRegisters { x }),
                0x65 => Ok(Opcode::LoadRegisters { x }),
                _ => Err(ParseOpcodeError::UnknownOpcode(opcode))
            }
            _ => Err(ParseOpcodeError::UnknownOpcode(opcode))
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::ClearScreen => write!(f, "CLR"),
            Opcode::Return => write!(f, "RET"),
            Opcode::Jump { addr } => write!(f, "JP {:#X}", addr),
            Opcode::Call { addr } => write!(f, "CALL {:#X}", addr),
            Opcode::SkipEqualByte { x, kk } => write!(f, "SE V{:X}, {:#X}", x, kk),
            Opcode::SkipNotEqualByte { x, kk } => write!(f, "SNE V{:X}, {:#X}", x, kk),
            Opcode::SkipEqual { x, y } => write!(f, "SE V{:X}, V{:X}", x, y),
            Opcode::SkipNotEqual { x, y } => write!(f, "SNE V{:X}, V{:X}", x, y),
            Opcode::LoadByte { x, kk } => write!(f, "LD V{:X}, {:#X}", x, kk),
            Opcode::AddByte { x, kk } => write!(f, "ADD V{:X}, {:#X}", x, kk),
            Opcode::Load { x, y } => write!(f, "LD V{:X}, V{:X}", x, y),
            Opcode::Or { x, y } => write!(f, "OR V{:X}, V{:X}", x, y),
            Opcode::And { x, y } => write!(f, "AND V{:X}, V{:X}", x, y),
            Opcode::Xor { x, y } => write!(f, "XOR V{:X}, V{:X}", x, y),
            Opcode::Add { x, y } => write!(f, "ADD V{:X}, V{:X}", x, y),
            Opcode::Sub { x, y } => write!(f, "SUB V{:X}, V{:X}", x, y),
            Opcode::Shr { x, y } => write!(f, "SHR V{:X}, V{:X}", x, y),
            Opcode::Shl { x, y } => write!(f, "SHL V{:X}, V{:X}", x, y),
            Opcode::Subn { x, y } => write!(f, "SUBN V{:X}, V{:X}", x, y),
            Opcode::LoadI { addr } => write!(f, "LD I, {:#X}", addr),
            Opcode::JumpV0 { addr } => write!(f, "JP V0, {:#X}", addr),
            Opcode::Random { x, kk } => write!(f, "RND V{:X}, {:#X}", x, kk),
            Opcode::Draw { x, y, n } => write!(f, "DRW V{:X}, V{:X}, {}", x, y, n),
            Opcode::SkipKeyPressed { x } => write!(f, "SKP V{:X}", x),
            Opcode::SkipKeyNotPressed { x } => write!(f, "SKNP V{:X}", x),
            Opcode::LoadDelayTimer { x } => write!(f, "LD V{:X}, DT", x),
            Opcode::WaitKeyPressed { x } => write!(f, "LD V{:X}, K", x),
            Opcode::LoadToDelayTimer { x } => write!(f, "LD DT, V{:X}", x),
            Opcode::LoadToSoundTimer { x } => write!(f, "LD ST, V{:X}", x),
            Opcode::AddI { x } => write!(f, "ADD I, V{:X}", x),
            Opcode::LoadSprite { x } => write!(f, "LD F, V{:X}", x),
            Opcode::LoadBCD { x } => write!(f, "LD B, V{:X}", x),
            Opcode::SaveRegisters { x } => write!(f, "LD [I], V{:X}", x),
            Opcode::LoadRegisters {x  } => write!(f, "LD V{:X}, [I]", x),
        }
    }
}

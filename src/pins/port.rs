use crate::registers::{digital::*, Register, RegisterCell};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Port {
    PD,
    PB,
    PC,
}

impl Port {
    pub const fn mode_reg(&self) -> RegisterCell<u8> {
        match self {
            Self::PD => DDRD::REGISTER,
            Self::PB => DDRB::REGISTER,
            Self::PC => DDRC::REGISTER,
        }
    }

    pub const fn out_reg(&self) -> RegisterCell<u8> {
        match self {
            Self::PD => PORTD::REGISTER,
            Self::PB => PORTB::REGISTER,
            Self::PC => PORTC::REGISTER,
        }
    }

    pub const fn in_reg(&self) -> RegisterCell<u8> {
        match self {
            Self::PD => PIND::REGISTER,
            Self::PB => PINB::REGISTER,
            Self::PC => PINC::REGISTER,
        }
    }
}

impl TryFrom<u8> for Port {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..8 => Ok(Self::PD),
            8..14 => Ok(Self::PB),
            14..20 => Ok(Self::PC),
            _ => Err("Invalid pin"),
        }
    }
}

#[repr(u8)]
pub enum ClockPolarity {
    RaisingEdge = 0,
    FallingEdge = 1,
}

#[repr(u8)]
pub enum CharacterSize {
    Size5 = 0,
    Size6 = 2,
    Size7 = 4,
    Size8 = 6,
}

#[repr(u8)]
pub enum StopBit {
    One = 0,
    Two = 8,
}

#[repr(u8)]
pub enum ParityMode {
    Disabled = 0,
    Even = 32,
    Odd = 48,
}

#[repr(u8)]
pub enum ModeSelect {
    Asynchronous = 0,
    Synchronous = 64,
    MasterSPI = 192,
}

pub(super) const DEFAULT_MODE: u8 = ClockPolarity::RaisingEdge as u8
    | CharacterSize::Size8 as u8
    | StopBit::One as u8
    | ParityMode::Disabled as u8
    | ModeSelect::Asynchronous as u8;

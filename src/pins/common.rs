pub const fn pin_to_bit(pin: u8) -> Result<u8, &'static str> {
    match pin {
        x @ 0..8 => Ok(x),
        x @ 8..14 => Ok(x - 8),
        x @ 14..20 => Ok(x - 14),
        _ => Err("Invalid pin"),
    }
}

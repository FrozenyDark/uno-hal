use crate::registers::*;

crate::INIT_REG!(DDRB: io8(0x04)); // Ports 8-13
crate::INIT_REG!(DDRC: io8(0x07)); // Ports 14-19
crate::INIT_REG!(DDRD: io8(0x0A)); // Ports 0-7

crate::INIT_REG!(PINB: io8(0x03));
crate::INIT_REG!(PINC: io8(0x06));
crate::INIT_REG!(PIND: io8(0x09));

crate::INIT_REG!(PORTB: io8(0x05));
crate::INIT_REG!(PORTC: io8(0x08));
crate::INIT_REG!(PORTD: io8(0x0B));

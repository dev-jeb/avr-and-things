//! no_std GPIO port abstraction for AVR (ATmega328P).
//!
//! Use Port B, C, and D via DDR/PORT/PIN registers:
//! - `PORTx`: data register (read/write)
//! - `DDRx`: data direction register (read/write)
//! - `PINx`: input pins (read only)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// A single AVR I/O port (DDR, PORT, PIN registers).
pub struct Port {
    ddr: *mut u8,
    port: *mut u8,
    pin: *const u8,
}

impl Port {
    /// Creates a port from register addresses. Unsafe: addresses must be valid for the target MCU.
    pub unsafe fn new(ddr_addr: u16, port_addr: u16, pin_addr: u16) -> Self {
        Port {
            ddr: ddr_addr as *mut u8,
            port: port_addr as *mut u8,
            pin: pin_addr as *const u8,
        }
    }

    /// Sets the direction of a pin (input or output).
    pub fn set_pin_mode(&mut self, pin: u8, is_output: bool) {
        unsafe {
            let current = read_volatile(self.ddr);
            let new = if is_output {
                current | (1 << pin)
            } else {
                current & !(1 << pin)
            };
            write_volatile(self.ddr, new);
        }
    }

    /// Sets the output state of a pin (high or low).
    pub fn set_pin_state(&mut self, pin: u8, is_high: bool) {
        unsafe {
            let current = read_volatile(self.port);
            let new = if is_high {
                current | (1 << pin)
            } else {
                current & !(1 << pin)
            };
            write_volatile(self.port, new);
        }
    }

    /// Reads the input state of a pin.
    pub fn read_pin(&self, pin: u8) -> bool {
        unsafe {
            let value = read_volatile(self.pin);
            (value & (1 << pin)) != 0
        }
    }
}

/// Port B (ATmega328P).
pub mod portb {
    use super::Port;

    const DDRB: u16 = 0x24;
    const PORTB: u16 = 0x25;
    const PINB: u16 = 0x23;

    /// Returns an initialized Port B handle.
    pub fn init() -> Port {
        unsafe { Port::new(DDRB, PORTB, PINB) }
    }
}

/// Port C (ATmega328P).
pub mod portc {
    use super::Port;

    const DDRC: u16 = 0x27;
    const PORTC: u16 = 0x28;
    const PINC: u16 = 0x26;

    /// Returns an initialized Port C handle.
    pub fn init() -> Port {
        unsafe { Port::new(DDRC, PORTC, PINC) }
    }
}

/// Port D (ATmega328P).
pub mod portd {
    use super::Port;

    const DDRD: u16 = 0x2a;
    const PORTD: u16 = 0x2b;
    const PIND: u16 = 0x29;

    /// Returns an initialized Port D handle.
    pub fn init() -> Port {
        unsafe { Port::new(DDRD, PORTD, PIND) }
    }
}

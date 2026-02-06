// src/port.rs

#![allow(dead_code)]

/*
    Here will be a module we can use to interact with the different Ports (Port B, Port C and Port D) of the atmega328p.

    Port: A general I/O port (GPIO — General Purpose Input/Output) is a pin (or set of pins) that the microcontroller can configure as either an input or an output. As output, the MCU drives the pin to a logic level (usually VCC = high or GND = low). In output mode the pin can act like a voltage source (or sink). As input, the MCU reads the voltage on the pin (high or low).

    Each Port on the atmega328p has three I/O registers associated to it.

    PORTx: data register (read/write)
    DDRx: data direction register (read/write)
    PINx: input pins (read only)

    THe DDxn (x = port (A,B,C), n = bit )

    Pull-ups: When a pin is configured as input and the internal pull-up is disabled, the pin is floating—its voltage is undefined, so reads can give high or low at different times. Enabling the pull-up resistor ties the pin weakly to VCC, so you get a stable logic high when nothing is actively pulling the pin low (e.g. an open switch to GND).
*/

use core::ptr::{read_volatile, write_volatile};

pub struct Port {
    ddr: *mut u8,
    port: *mut u8,
    pin: *const u8,
}

impl Port {
    pub unsafe fn new(ddr_addr: u16, port_addr: u16, pin_addr: u16) -> Self {
        Port {
            ddr: ddr_addr as *mut u8,
            port: port_addr as *mut u8,
            pin: pin_addr as *const u8,
        }
    }

    /// Sets the direction of a pin (input or output)
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

    /// Sets the output state of a pin (high or low)
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

    /// Reads the input state of a pin
    pub fn read_pin(&self, pin: u8) -> bool {
        unsafe {
            let value = read_volatile(self.pin);
            (value & (1 << pin)) != 0
        }
    }
}

pub mod portb {
    use super::Port;

    const DDRB: u16 = 0x24;
    const PORTB: u16 = 0x25;
    const PINB: u16 = 0x23;

    pub fn init() -> Port {
        unsafe { Port::new(DDRB, PORTB, PINB) }
    }
}

pub mod portc {
    use super::Port;

    const DDRC: u16 = 0x27;
    const PORTC: u16 = 0x28;
    const PINC: u16 = 0x26;

    pub fn init() -> Port {
        unsafe { Port::new(DDRC, PORTC, PINC) }
    }
}

pub mod portd {
    use super::Port;

    const DDRD: u16 = 0x2a;
    const PORTD: u16 = 0x2b;
    const PIND: u16 = 0x29;

    pub fn init() -> Port {
        unsafe { Port::new(DDRD, PORTD, PIND) }
    }
}

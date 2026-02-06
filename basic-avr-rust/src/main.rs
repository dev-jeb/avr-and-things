#![no_std]
#![no_main]

use avr_port::portb;

#[panic_handler]
pub fn panic(_panic: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn busy_delay(cycles: u32) {
    for _ in 0..cycles {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let mut port_b = portb::init();
    port_b.set_pin_mode(0x05, true);

    loop {
        port_b.set_pin_state(0x05, true);
        busy_delay(100_000);
        port_b.set_pin_state(0x05, false);
        busy_delay(100_000);
    }
}

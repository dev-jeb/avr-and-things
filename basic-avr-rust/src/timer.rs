// src/timer.rs

pub struct Timer8Bit {
    /// Timer control register A
    pub counter: *mut u8,
    /// Timer control register A
    pub control_register_a: *mut u8 = 0x25,
    /// Timer output compare register A
    pub output_compare_register_a: *mut u8 = 0x24,
    /// Timer output compare register B
    pub output_compare_register_b: *mut u8 = 0x23,
    /// Timer interrupt flag register
    pub interrupt_flag_register: *mut u8,
    /// Timer interrupt mask register
    pub interrupt_mask_register: *mut u8,
}

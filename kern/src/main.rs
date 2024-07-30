#![no_std]
#![feature(prelude_2024)]
#![feature(alloc_error_handler)]
// #![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(auto_traits)]
// #![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(negative_impls)]

#[cfg(not(test))]
mod init;

pub mod console;
pub mod mutex;
pub mod shell;

use console::kprintln;
use core::unimplemented;
use core::arch::asm;

// FIXME: You need to add dependencies here to
// test your drivers (Phase 2). Add them as needed.

// fn kmain() -> ! {
//     // FIXME: Start the shell.
//     kprintln!("hey");
//     loop {}
// }

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 6000) {
        unsafe {
            asm!("nop");
        }
    }
}


use pi::timer::spin_sleep;
use core::time::Duration;

#[no_mangle]
unsafe fn kmain() -> ! {
    // FIXME: STEP 1: Set GPIO Pin 16 as output.
    GPIO_FSEL1.write_volatile((GPIO_FSEL1.read_volatile() & !(0b111 << 18)) | (0b001 << 18));

    // FIXME: STEP 2: Continuously set and clear GPIO 16.
    let delay = Duration::from_secs(5);
    loop {
        GPIO_SET0.write_volatile(1 << 16);
        spin_sleep(delay);
        GPIO_CLR0.write_volatile(1 << 16);
        spin_sleep(delay);
    }
}

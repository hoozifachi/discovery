#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::entry;
use aux7::{iprintln, ITM};

// Print the current content of odr
fn iprint_odr(itm: &mut ITM) {
    const GPIOE_ODR: u32 = 0x4800_1014;

    unsafe {
        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read(GPIOE_ODR as *const i16)
        );
    }
}

#[entry]
fn main() -> ! {
    let mut itm = aux7::init().0;

    unsafe {
        // A magic addess
        const GPIOE_BSSR: u32 = 0x4800_1018;

        // Print the initial contents of ODR
        iprint_odr(&mut itm);

        // turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSSR as *mut u32, 1 << 9);
        iprint_odr(&mut itm);

        // turn on the "East" LED (green)
        ptr::write_volatile(GPIOE_BSSR as *mut u32, 1 << 11);
        iprint_odr(&mut itm);

        // turn off the "North" LED (red)
        ptr::write_volatile(GPIOE_BSSR as *mut u32, 1 << (9 + 16));
        iprint_odr(&mut itm);

        // turn off the "East" LED (green)
        ptr::write_volatile(GPIOE_BSSR as *mut u32, 1 << (11 + 16));
        iprint_odr(&mut itm);
    }

    loop {}
}

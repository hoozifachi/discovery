#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::entry;

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    // turn on the "North" LED (red)
    gpioe.bsrr.write(|w| w.bs9().set_bit());

    // turn on the "East" LED (green)
    gpioe.bsrr.write(|w| w.bs11().set_bit());

    // turn off the "North" LED (red)
    gpioe.bsrr.write(|w| w.br9().set_bit());

    // turn off the "East" LED (green)
    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}

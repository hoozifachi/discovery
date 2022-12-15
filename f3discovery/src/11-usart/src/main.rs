#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    let str = "The quick brown fox jumps over the lazy dog.";

    for c in str.bytes() {
        // Send a single character
        usart1.tdr.write(|w| w.tdr().bits(u16::from(c)));
    }

    loop {}
}

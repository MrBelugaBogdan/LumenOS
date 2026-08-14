#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let _ = hprintln!("Hello, LumenOS!");

    // Ядро завжди працює в нескінченному циклі
    loop {}
}

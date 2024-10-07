//% FEATURES: embedded-hal-02

#![no_std]
#![no_main]


use esp_lp_hal::{delay::Delay, prelude::*};
use panic_halt as _;


        const ADDRESS: u32 = 0x400;


#[entry]
fn main() -> ! {
    let mut i: u32 = 0;

    let ptr = ADDRESS as *mut u32;

    loop {
        i = i.wrapping_add(1u32);
        unsafe {
            ptr.write_volatile(i);

        }
        Delay.delay_millis(100);
    
    }
}
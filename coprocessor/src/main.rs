#![no_std]
#![no_main]

use ef_led::run;
use esp_lp_hal::{delay::Delay, gpio::Output, prelude::*};
use panic_halt as _;

mod ef_led;

const ADDRESS: u32 = 0x20;

#[entry]
fn main(mut gpio21: Output<21>, mut gpio9: Output<9>) -> ! {
    // enable boost converter to drive the leds
    gpio9.set_output(true);

    let ptr = ADDRESS as *mut u32;
    unsafe {
        ptr.write_volatile(122);
    }

    Delay.delay_millis(200);
    loop {
        unsafe {
            ptr.write_volatile(111111);
        }

        for i in 0..20 {
            unsafe {
                ptr.write_volatile(i as u32);
            }
            run(&mut gpio21, &[(i, 0, 0); 17]);
            Delay.delay_millis(10);
        }

        for i in 0..20 {
            unsafe {
                ptr.write_volatile(30 + i as u32);
            }
            run(&mut gpio21, &[(0, i, 0); 17]);

            Delay.delay_millis(10);
        }

        for i in 0..20 {

            unsafe {
                ptr.write_volatile(50 + i as u32);
            }
            run(&mut gpio21, &[(0, 0, i); 17]);

            Delay.delay_millis(10);
        }
    }
}

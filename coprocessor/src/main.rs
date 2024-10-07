#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use esp_lp_hal::{delay::Delay, gpio::Output, prelude::*};
use panic_halt as _;

const ADDRESS: u32 = 0x400;

// #define EFLED_PIN_LED_DATA 21
// #define EFLED_PIN_5VBOOST_ENABLE 9

#[entry]
fn main(mut gpio21: Output<21>) -> ! {
    // let mut i: u32 = 0;

    let ptr = ADDRESS as *mut u32;
    unsafe {
        ptr.write_volatile(122);
    }

    loop {
        for s in 1..1000000 {
            Delay.delay_nanos(500000);
            let _ = gpio21.set_low();
            unsafe {
                ptr.write_volatile(s);
            }
            for _ in 0..6 {
                for _ in 0..8 {
                    let _ = gpio21.set_high();

                    Delay.delay_nanos(260);

                    let _ = gpio21.set_low();
                    Delay.delay_nanos(700);
                }
            }
            unsafe {
                ptr.write_volatile(20000000 + s);
            }
        }

        // for _ in 0..6 {
        //     for _ in 0..8 {
        //         gpio21.set_high();
        //         Delay.delay_nanos(395);
        //         gpio21.set_low();
        //         Delay.delay_nanos(835);
        //     }
        // }

        // for i in 0..3 {
        //     for j in 0..6 {
        //         gpio1.set_high();
        //         Delay.delay_nanos(375);
        //         gpio1.set_low();
        //         Delay.delay_nanos(825);
        //     }
        //     gpio1.set_high();
        //     Delay.delay_nanos(775);
        //     gpio1.set_low();
        //     Delay.delay_nanos(425);

        //     for j in 0..7{
        //         gpio1.set_high();
        //         Delay.delay_nanos(375);
        //         gpio1.set_low();
        //         Delay.delay_nanos(825);
        //     }

        //     for j in 0..7 {
        //         gpio1.set_high();
        //         Delay.delay_nanos(375);
        //         gpio1.set_low();
        //         Delay.delay_nanos(825);
        //     }
        // }
    }
}

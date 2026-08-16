#![no_std]
#![no_main]
mod animation;
mod ef_led;
mod led_data;

use esp_lp_hal::{delay::Delay, gpio::Output, prelude::*};
use panic_halt as _;

use crate::animation::Animator;

// TODO: figure out why those are the wrong values.
unsafe extern "C" {
    static __shared_start: u8;
    static __shared_length: u8;
}

const RAM_END: usize = 1024 * 8;
const SHARED_LENGTH: usize = 32;
const SHARED_START: usize = RAM_END - SHARED_LENGTH;

const DEBUG_WORD_ADDR: usize = SHARED_START;
const DEBUG_WORD: *mut u32 = DEBUG_WORD_ADDR as *mut u32;

// Packed bits (from MSB to LSB) : (pattern_settings: u16, pattern: u8, intensity: u8)
const SETTINGS_ADDR: usize = SHARED_START + 4;
const SETTINGS: *mut u32 = SETTINGS_ADDR as *mut u32;

#[entry]
fn main(gpio21: Output<21>, mut gpio9: Output<9>) -> ! {
    // enable boost converter to drive the leds
    gpio9.set_output(true);

    unsafe {
        DEBUG_WORD.write_volatile(122);
        // SETTINGS.write_volatile(0);
    }
    // let mut controller = Control::new(gpio21);
    // let black = (0, 0, 0);
    // let white = (255, 255, 255);
    // let pink = (230, 0, 100);
    // let l_blue = (100, 100, 230);

    let mut animator = Animator::new(gpio21);

    // Delay.delay_millis(200);
    let tick_delay_ms = 0;
    loop {
        let settings = unsafe { SETTINGS.read_volatile() };
        let intensity = settings as u8;
        let pattern = (settings >> 8) as u8;
        let pattern_settings = (settings >> 16) as u16;

        animator.tick(intensity, pattern, pattern_settings);
        Delay.delay_millis(tick_delay_ms);

        unsafe {
            DEBUG_WORD.write_volatile(intensity.into());
        }

        // controller.set_intensity(intensity);

        // pride flag trans

        // controller.set_data(&[
        //     black, black, black, black, white, white, l_blue, l_blue, pink, pink, white, white,
        //     white, pink, pink, l_blue, l_blue,
        // ]);

        // Delay.delay_millis(200);
        // controller.set_index(0, l_blue);
        // Delay.delay_millis(200);
        // controller.set_index(1, pink);
        // Delay.delay_millis(200);
        // controller.set_index(2, pink);
        // Delay.delay_millis(200);
    }
}

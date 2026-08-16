#![no_std]
#![no_main]
mod ef_led;

use ef_led::{run, LEDData, Rgb, LED_COUNT};
use esp_lp_hal::{delay::Delay, gpio::Output, prelude::*};
use panic_halt as _;

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

const SETTINGS_ADDR: usize = SHARED_START + 4;
const SETTINGS: *mut u32 = SETTINGS_ADDR as *mut u32;

#[entry]
fn main(gpio21: Output<21>, mut gpio9: Output<9>) -> ! {
    // enable boost converter to drive the leds
    gpio9.set_output(true);

    unsafe {
        DEBUG_WORD.write_volatile(122);
        SETTINGS.write_volatile(0);
    }
    let mut controller = Control::new(gpio21);
    let black = (0, 0, 0);
    let white = (255, 255, 255);
    let pink = (230, 0, 100);
    let l_blue = (100, 100, 230);

    Delay.delay_millis(200);
    loop {
        let settings = unsafe {
            DEBUG_WORD.write_volatile(111111);
            SETTINGS.read_volatile()
        };
        let intensity = settings as u8;

        controller.set_intensity(intensity);

        // pride flag trans

        controller.set_data(&[
            black, black, black, black, white, white, l_blue, l_blue, pink, pink, white, white,
            white, pink, pink, l_blue, l_blue,
        ]);

        Delay.delay_millis(200);
        controller.set_index(0, l_blue);
        Delay.delay_millis(200);
        controller.set_index(1, pink);
        Delay.delay_millis(200);
        controller.set_index(2, pink);
        Delay.delay_millis(200);
    }
}

struct Control<const PIN: u8> {
    current_data: LEDData,
    output: Output<PIN>,
    intensity: u8,
}

impl<const PIN: u8> Control<PIN> {
    pub fn new(output: Output<PIN>) -> Self {
        Self {
            current_data: [(0, 0, 0); LED_COUNT],
            output,
            intensity: 30,
        }
    }

    pub fn apply(&mut self) {
        run(
            &mut self.output,
            &apply_intensity(&self.current_data, self.intensity),
        );
    }

    pub fn fill(&mut self, col: Rgb) {
        self.current_data = [col; LED_COUNT];
        self.apply();
    }

    pub fn set_index(&mut self, index: usize, col: Rgb) {
        if index < LED_COUNT {
            self.current_data[index] = col
        }
        self.apply();
    }

    pub fn set_data(&mut self, data: &LEDData) {
        self.current_data = *data;
        self.apply();
    }

    pub fn set_intensity(&mut self, intensity: u8) {
        self.intensity = intensity;
        self.apply();
    }
}

fn apply_intensity_element(value: u8, intensity: u8) -> u8 {
    ((value as u32 * 1000 * intensity as u32) / 255000) as u8
}

fn apply_intensity(current_data: &LEDData, intensity: u8) -> LEDData {
    let mut out = *current_data;

    for (i, v) in current_data.iter().enumerate() {
        out[i] = (
            apply_intensity_element(v.0, intensity),
            apply_intensity_element(v.1, intensity),
            apply_intensity_element(v.2, intensity),
        )
    }
    out
}

#![no_std]
#![no_main]

use ef_led::{run, LEDData, Rgb, LED_COUNT};
use esp_lp_hal::{delay::Delay, gpio::Output, prelude::*};
use panic_halt as _;

mod ef_led;

const ADDRESS: u32 = 0x20;

#[entry]
fn main(gpio21: Output<21>, mut gpio9: Output<9>) -> ! {
    // enable boost converter to drive the leds
    gpio9.set_output(true);

    let ptr = ADDRESS as *mut u32;
    unsafe {
        ptr.write_volatile(122);
    }
    let mut controller = Control::new(gpio21);
    let black = (0, 0, 0);
    let white = (255, 255, 255);
    let pink = (230, 0, 100);
    let l_blue = (100, 100 ,230);

    Delay.delay_millis(200);
    loop {
        for i in 5..20 {
            unsafe {
                ptr.write_volatile(111111);
            }
            controller.set_intensity(i);

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
    ((value as u32 * 100 * intensity as u32) / 25500) as u8
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

mod led;

use std::{thread::sleep, time::Duration};

use esp_idf_hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::Peripherals;
use smart_leds::RGB8;

use crate::led::EFLed;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let led_pin = peripherals.pins.gpio21;
    let channel = peripherals.rmt.channel0;
    let boost_enable = peripherals.pins.gpio9;
    let mut ef_led =
        EFLed::try_init(channel, led_pin, PinDriver::output(boost_enable).unwrap()).unwrap();
    log::info!("Hello, world!");

    ef_led.turn_on();
    // sleep(Duration::from_millis(1));
    ef_led.fill(RGB8::new(0, 0, 0));
    sleep(Duration::from_millis(1000));

    loop {
        ef_led.fill(RGB8::new(0, 255, 0));
        ef_led.write_state();
        for i in 0..50 {
            ef_led.set_intensity(i);
            sleep(Duration::from_millis(10))
        }
        for i in 0..50 {
            ef_led.set_intensity(50 - i);
            sleep(Duration::from_millis(10))
        }

        ef_led.fill(RGB8::new(255, 0, 0));
        ef_led.write_state();
        for i in 0..50 {
            ef_led.set_intensity(i);
            sleep(Duration::from_millis(10))
        }
        for i in 0..50 {
            ef_led.set_intensity(50 - i);
            sleep(Duration::from_millis(10))
        }
    }
}

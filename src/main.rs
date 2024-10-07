mod led;

use std::{default, thread::sleep, time::Duration};

use esp_idf_hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::Peripherals;
use smart_leds::RGB8;

use crate::led::EFLed;
use anyhow::Result;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::wifi::{AccessPointConfiguration, AuthMethod, BlockingWifi, EspWifi, Protocol};
use esp_idf_sys::nvs_flash_init;

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    unsafe { nvs_flash_init(); }

    
    let sysloop = EspSystemEventLoop::take()?;

    
    

    let peripherals = Peripherals::take().unwrap();
    let led_pin = peripherals.pins.gpio21;
    let channel = peripherals.rmt.channel0;
    let boost_enable = peripherals.pins.gpio9;
    let mut ef_led =
        EFLed::try_init(channel, led_pin, PinDriver::output(boost_enable).unwrap()).unwrap();
    log::info!("Hello, world!");

    let modem = peripherals.modem;
    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;
    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;

    let wifi_config = esp_idf_svc::wifi::Configuration::AccessPoint(AccessPointConfiguration {
        channel: 4,
        auth_method: AuthMethod::None,
        password: "".try_into().expect("Could not parse SSID into WiFi config"),
        ssid: "Awawi".parse().expect("Could not parse Wifi password"),
        max_connections: 10,
        ..Default::default()
    });
    wifi.set_configuration(&wifi_config)?;
    wifi.start()?;


    log::info!("freq: {}", unsafe{esp_idf_hal::sys::esp_rom_get_cpu_ticks_per_us()});

    ef_led.turn_on();
    // sleep(Duration::from_millis(1));
    ef_led.fill(RGB8::new(0, 0, 0));
    sleep(Duration::from_millis(1000));

    loop {
        ef_led.fill(RGB8::new(0, 255, 0));
        for i in 0..50 {
            ef_led.set_intensity(i);
            sleep(Duration::from_millis(10))
        }
        for i in 0..50 {
            ef_led.set_intensity(50 - i);
            sleep(Duration::from_millis(10))
        }

        ef_led.fill(RGB8::new(255, 0, 0));
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

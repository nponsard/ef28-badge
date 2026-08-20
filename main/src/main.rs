#![no_main]
#![no_std]

mod buttons;
mod drawer;
mod leds_controller;
mod pins;
mod screens;

use ariel_os::{
    gpio, hal,
    log::info,
    spi::{self, main::SpiDevice},
    time::Delay,
};
use drawer::{DisplayController, DisplayTarget};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, pubsub::PubSubChannel, watch::Watch,
};

static WATCH: Watch<CriticalSectionRawMutex, ([u8; 4736], bool), 1> = Watch::new();

#[ariel_os::task(autostart, peripherals)]
async fn screen(peripherals: pins::Epd) {
    static SPI_BUS: once_cell::sync::OnceCell<
        Mutex<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, hal::spi::main::Spi>,
    > = once_cell::sync::OnceCell::new();

    info!("Starting EPD demo");
    let mut spi_config = hal::spi::main::Config::default();
    spi_config.frequency = const {
        spi::main::highest_freq_in(spi::main::Kilohertz::MHz(1)..=spi::main::Kilohertz::MHz(20))
    };

    info!("Configured SPI");

    let spi_bus = pins::EpdSpi::new(
        peripherals.spi_sck,
        peripherals.spi_miso,
        peripherals.spi_mosi,
        spi_config,
    );

    info!("Created SPI bus");

    let _ = SPI_BUS.set(Mutex::new(spi_bus));

    let cs_output: gpio::Output = gpio::Output::new(peripherals.spi_cs, gpio::Level::High);
    let dc = gpio::Output::new(peripherals.dc, gpio::Level::High);
    let busy = gpio::Input::builder(peripherals.busy, gpio::Pull::Up)
        .build_with_interrupt()
        .unwrap();
    let reset = gpio::Output::new(peripherals.reset, gpio::Level::High);

    let spi_device = SpiDevice::new(SPI_BUS.get().unwrap(), cs_output);

    let config = ssd1680_rs::config::DisplayConfig::epd_290_t94();

    let mut epd_controller: ssd1680_rs::driver_async::SSD1680<_, _, _, _, _> =
        ssd1680_rs::driver_async::SSD1680::new(reset, dc, busy, Delay, spi_device, config);

    epd_controller.hw_init().await.unwrap();

    let sender = WATCH.sender();
    let receiver = WATCH.receiver().unwrap();

    let mut manager = DisplayController::new(epd_controller, receiver);

    let mut draw_target = DisplayTarget::new(sender);

    embassy_futures::join::join(manager.run(), screens::screen(&mut draw_target)).await;
}

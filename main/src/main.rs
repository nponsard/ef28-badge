#![no_main]
#![no_std]

mod drawer;
mod pins;

use ariel_os::{
    gpio, hal,
    log::info,
    spi::{self, main::SpiDevice},
    time::{Delay, Timer},
};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, pubsub::PubSubChannel, watch::Watch,
};
use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};

use drawer::{DisplayController, DisplayTarget};

use esp_hal::{gpio::rtc_io::LowPowerOutput, load_lp_code};

static WATCH: Watch<CriticalSectionRawMutex, [u8; 4736], 1> = Watch::new();

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Ulp) {
    info!(
        "Hello from main()! Running on a {} board.",
        ariel_os::buildinfo::BOARD
    );

    let boost = LowPowerOutput::new(peripherals.boost);
    let pin = LowPowerOutput::new(peripherals.smart_led);

    let mut ulp_core = esp_hal::ulp_core::UlpCore::new(peripherals.ulp);

    ulp_core.stop();
    info!("ulp core stopped");

    // load code to LP core
    let lp_core_code =
        load_lp_code!("../coprocessor/target/riscv32imc-unknown-none-elf/release/coprocessor");

    // start LP core
    lp_core_code.run(
        &mut ulp_core,
        esp_hal::ulp_core::UlpCoreWakeupSource::HpCpu,
        pin,
        boost,
    );
    info!("ulpcore run");

    let data = (0x5000_0020) as *mut u32;

    // let mut rtc = Rtc::new(peripherals.LPWR);
    // rtc.sleep_deep(&[&ULPWake {}, ]);

    loop {
        info!("Current debug code {}", unsafe { data.read_volatile() });
        Timer::after_millis(300).await;
    }
}

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

    info!("entering main loop");

    embassy_futures::join::join(manager.run(), async {
        loop {
            info!("Display loop");
            // Off = black
            draw_target.clear(BinaryColor::Off);
            draw_target.flush();

            Timer::after_millis(300).await;
            draw_target.clear(BinaryColor::On);
            draw_target.flush();
        }
    })
    .await;
}

#![no_std]
#![no_main]
use alloc::vec;
use embedded_hal::spi::SpiBus;
use esp_backtrace as _;
use esp_hal::gpio::rtc_io::LowPowerOutput;
use esp_hal::prelude::_fugit_RateExtU32;
use esp_hal::spi::SpiBitOrder;
use esp_hal::uart::config::Config;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Io, Level, Output},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};
use log::info;

extern crate alloc;
use core::mem::MaybeUninit;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let mut clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    init_heap();

    esp_println::logger::init_logger_from_env();

    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0, &clocks);
    let _init = esp_wifi::initialize(
        esp_wifi::EspWifiInitFor::Wifi,
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
        &clocks,
    )
    .unwrap();

    let io: Io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let boost = LowPowerOutput::new(io.pins.gpio9);
    let pin = LowPowerOutput::new(io.pins.gpio21);

    let mut ulp_core = esp_hal::ulp_core::UlpCore::new(peripherals.ULP_RISCV_CORE);

    ulp_core.stop();
    info!("ulp core stopped");

    // boost.set_high();

    // let mut spi = esp_hal::spi::master::Spi::new(
    //     peripherals.SPI2,
    //     5.MHz(),
    //     esp_hal::spi::SpiMode::Mode0,
    //     &clocks,
    // )
    // .with_bit_order(SpiBitOrder::MSBFirst, SpiBitOrder::MSBFirst)
    // .with_mosi(io.pins.gpio21);

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
    loop {  
        info!("Current      {}", unsafe { data.read_volatile() });
        // spi.write(&[
        //     0b11011011, 0b01101101, 0b10110110, 0b11011011, 0b01101101, 0b10110110, 0b11011011,
        //     0b01101101, 0b10110110, 0b11011011, 0b01101101, 0b10110110, 0b11011011, 0b01101101,
        //     0b10110110, 0b11011011, 0b01101101, 0b10110110, 0b11011011, 0b01101101, 0b10110110,
        //     0b11011011, 0b01101101, 0b10110110, 0b11011011, 0b01101101, 0b10110110, 0b11011011,
        //     0b01101101, 0b10110110, 0b11011011, 0b01101101, 0b10110110, 0b11011011, 0b01101101,
        //     0b10110110, 0b11011011, 0b01101101, 0b10110110, 0b11011011, 0b01101101, 0b10110110,
        //     0b11011011, 0b01101101, 0b10110110, 0b11011011, 0b01101101, 0b10110110, 0b11011011,
        //     0b01101101, 0b10110110, 0b11011011, 0b01101101, 0b10110110, 0b11011011, 0b01101101,
        //     0b10110110, 0b11011011, 0b01101101, 0b10110110, 0b11011011, 0b01101101, 0b10110110,
        //     0b11011011, 0b01101101, 0b10110110, 0b11011011, 0b01101101, 0b10110110, 0b11011011,
        //     0b01101101, 0b10110110, 0b11011011, 0b01101101, 0b10110110, 0b11011011, 0b01101101,
        //     0b10110110, 0b11011011, 0b01101101, 0b10110110, 0b11011011, 0b01101101, 0b10110110,
        //     0b11011011, 0b01101101, 0b10110110, 0b11011011, 0b01101101, 0b10110110,
        // ]);
        // spi.flush();
        delay.delay_millis(300);
    }
}

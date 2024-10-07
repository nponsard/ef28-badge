#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{rtc_io::LowPowerOutput, Io, Level, Output},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    ulp_core,
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

    let clocks = ClockControl::max(system.clock_control).freeze();
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

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio9, Level::High);
    led.set_high();
    let pin = LowPowerOutput::new(io.pins.gpio21);

    let mut ulp_core = ulp_core::UlpCore::new(peripherals.ULP_RISCV_CORE);

    ulp_core.stop();
    log::info!("ulp core stopped ?");

    // load code to LP core
    let lp_core_code =
        load_lp_code!("../coprocessor/target/riscv32imc-unknown-none-elf/release/coprocessor");

    // start LP core
    lp_core_code.run(&mut ulp_core, ulp_core::UlpCoreWakeupSource::HpCpu, pin);
    info!("ulpcore run aa");

    let data = (0x5000_0400) as *mut u32;
    loop {
        info!("Current aaa {}", unsafe {
            data.read_volatile()
        });
        delay.delay_millis(10);
    }
}

use ariel_os::{log::info, time::Timer};
use esp_hal::{gpio::rtc_io::LowPowerOutput, load_lp_code};

use crate::pins;

const RTC_START: usize = 0x5000_0000;
const RTC_LENGTH: usize = 8 * 1024;

const SHARED_LENGTH: usize = 32;
const SHARED_START: usize = RTC_START + RTC_LENGTH - SHARED_LENGTH;

const DEBUG_WORD_ADDR: usize = SHARED_START;
const DEBUG_WORD: *mut u32 = DEBUG_WORD_ADDR as *mut u32;

const SETTINGS_ADDR: usize = SHARED_START + 4;
const SETTINGS: *mut u32 = SETTINGS_ADDR as *mut u32;

#[ariel_os::task(autostart, peripherals)]
async fn ulp_setup(peripherals: pins::Ulp) {
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

    // let data = (0x5000_0000 + 8*1024 - 32) as *mut u32;
    // let mut rtc = Rtc::new(peripherals.LPWR);
    // rtc.sleep_deep(&[&ULPWake {}, ]);

    loop {
        info!("Current debug code {}", unsafe {
            DEBUG_WORD.read_volatile()
        });
        Timer::after_secs(50).await;
    }
}

pub fn write_settings(intensity: u8, pattern: u8, pattern_setting: u16) {
    let word = intensity as u32 | (pattern as u32) << 8 | (pattern_setting as u32) << 16;
    unsafe { SETTINGS.write_volatile(word) };
}

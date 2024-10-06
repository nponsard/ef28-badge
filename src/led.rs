use esp_idf_hal::gpio::{Output, OutputPin, Pin, PinDriver};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::rmt::RmtChannel;
use esp_idf_sys::EspError;
use log::info;
use smart_leds::RGB8;
use smart_leds_trait::SmartLedsWrite;
use thiserror::Error;
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrb24;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, Ws2812Esp32RmtDriverError};

const LED_COUNT: usize = 17;

#[derive(Error, Debug)]
pub enum EFLedError {
    /// Too bright for safe use (converter is not designed to run all the leds at full power)
    #[error("Intensity of the led is too bright for the circuits")]
    IntensityTooHigh,
    #[error("Driver error : {0}")]
    Ws2812(#[from] Ws2812Esp32RmtDriverError),
    #[error("Esp error : {0}")]
    Esp(#[from] EspError),
}

pub struct EFLed<'a, BoostPin: Pin> {
    leds: [RGB8; LED_COUNT],
    intensity: u8,
    ws2812: LedPixelEsp32Rmt<'a, RGB8, LedPixelColorGrb24>,
    powered: bool,
    boost_pin_driver: PinDriver<'a, BoostPin, Output>,
}

impl<'a, BoostPin: Pin> EFLed<'a, BoostPin> {
    pub fn try_init(
        channel: impl Peripheral<P = impl RmtChannel> + 'a,
        pin: impl Peripheral<P = impl OutputPin> + 'a,
        boost_pin: PinDriver<'a, BoostPin, Output>,
    ) -> Result<Self, Ws2812Esp32RmtDriverError> {
        Ok(EFLed {
            intensity: 10,
            powered: false,
            leds: [RGB8::new(0, 0, 0); LED_COUNT],
            ws2812: LedPixelEsp32Rmt::<RGB8, LedPixelColorGrb24>::new(channel, pin)?,
            boost_pin_driver: boost_pin,
        })
    }

    pub fn turn_on(&mut self) -> Result<(), EFLedError> {
        self.boost_pin_driver.set_high()?;

        self.powered = true;
        Ok(())
    }

    pub fn turn_off(&mut self) -> Result<(),EFLedError>
    {
        self.boost_pin_driver.set_low()?;
        self.powered = false;
        Ok(())
    }

    pub fn set_intensity(&mut self, intensity: u8) -> Result<(), EFLedError> {
        if intensity > 50 {
            return Err(EFLedError::IntensityTooHigh);
        }
        self.intensity = intensity;
        self.write_state()
    }

    pub fn fill(&mut self, col: RGB8) -> Result<(), EFLedError> {
        info!("fill : {:?}", col);
        self.leds = [col; LED_COUNT];
        self.write_state()
    }
    pub fn write_state(&mut self) -> Result<(), EFLedError> {
        let transformed = apply_intensity(&self.intensity, &self.leds);
        log::info!("transformed : {:?}", transformed);
        self.ws2812.write(transformed.into_iter())?;
        Ok(())
    }
}

fn apply_intensity<const COUNT: usize>(intensity: &u8, input: &[RGB8; COUNT]) -> [RGB8; COUNT] {
    input.map(|i| {
        i.iter()
            .map(|c| (c as u16 * *intensity as u16 / 255) as u8)
            .collect()
    })
}

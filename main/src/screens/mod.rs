mod presentation;
mod settings;

use core::fmt::Debug;

use ariel_os::log::{Debug2Format, info};
use embedded_graphics::pixelcolor::BinaryColor;
use tinybmp::Bmp;
use u8g2_fonts::{FontRenderer, fonts};

use crate::{buttons::clear_inputs, drawer::DisplayTarget};

pub const PFP: Bmp<'_, BinaryColor> = match Bmp::from_slice(include_bytes!("../../assets/pfp.bmp"))
{
    Ok(bmp) => bmp,
    Err(_) => panic!("Cannot parse bmp"),
};

const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_maniac_tr>();

const MEDIUM_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvR12_tr>();
const MEDIUM_SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_crox2h_tf>();

const MENU_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_crox1h_tf>();

const SMALL_MONOSPACE_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_spleen5x8_mr>();
#[derive(Debug)]
pub enum Screen {
    Presentation,
    SettingsMain,
    SettingsLed,
}

impl Screen {
    pub async fn run(&self, draw_target: &mut DisplayTarget<'_>) -> Self {
        info!("Launching {:?}", Debug2Format(&self));
        match self {
            Self::Presentation => presentation::presentation(draw_target).await,
            Self::SettingsMain => settings::settings_main(draw_target).await,
            Screen::SettingsLed => settings::settings_led(draw_target).await,
        }
    }
}

pub async fn screen(draw_target: &mut DisplayTarget<'_>) {
    let mut selected = Screen::Presentation;

    loop {
        clear_inputs();
        selected = selected.run(draw_target).await
    }
}

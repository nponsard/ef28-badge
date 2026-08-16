mod presentation;

use ariel_os::{log::info, time::Timer};
use embedded_graphics::{
    Drawable as _, draw_target::DrawTarget, geometry::Point, image::Image, pixelcolor::BinaryColor,
};
use tinybmp::Bmp;
use u8g2_fonts::{
    FontRenderer, fonts,
    types::{FontColor, HorizontalAlignment, VerticalPosition},
};

use crate::drawer::DisplayTarget;

pub const PFP: Bmp<'_, BinaryColor> = match Bmp::from_slice(include_bytes!("../../assets/pfp.bmp")) {
    Ok(bmp) => bmp,
    Err(_) => panic!("Cannot parse bmp"),
};

const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_maniac_tr>();

const MEDIUM_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvR12_tr>();
const MEDIUM_SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_crox2h_tf>();

const SMALL_MONOSPACE_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_spleen5x8_mr>();

pub enum Screen {
    Presentation
}

impl Screen {
    pub  async fn run(&self ,draw_target: &mut DisplayTarget<'_>)-> Self {
        match self {
            _ => presentation::presentation(draw_target).await,
        }
    }
}

pub async fn screen(draw_target: &mut DisplayTarget<'_>) {
    let mut selected = Screen::Presentation;

    loop {
        selected = selected.run(draw_target)
    }
}

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

const PFP: Bmp<'_, BinaryColor> = match Bmp::from_slice(include_bytes!("../../assets/pfp.bmp")) {
    Ok(bmp) => bmp,
    Err(_) => panic!("Cannot parse bmp"),
};

const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_maniac_tr>();

const MEDIUM_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvR12_tr>();
const MEDIUM_SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_crox2h_tf>();

const SMALL_MONOSPACE_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_spleen5x8_mr>();



pub async fn screen(mut draw_target: DisplayTarget<'_>) {
    draw_target.clear(BinaryColor::On);
    Image::new(&PFP, Point::zero())
        .draw(&mut draw_target)
        .unwrap();

    BIG_FONT
        .render_aligned(
            "sautax",
            Point::new(64, 160),
            VerticalPosition::Baseline,
            HorizontalAlignment::Center,
            FontColor::Transparent(BinaryColor::Off),
            &mut draw_target,
        )
        .unwrap();

    MEDIUM_FONT
        .render_aligned(
            "They/Them",
            Point::new(64, 180),
            VerticalPosition::Baseline,
            HorizontalAlignment::Center,
            FontColor::Transparent(BinaryColor::Off),
            &mut draw_target,
        )
        .unwrap();

    MEDIUM_SMALL_FONT
        .render_aligned(
            "OC: Emery",
            Point::new(0, 210),
            VerticalPosition::Baseline,
            HorizontalAlignment::Left,
            FontColor::Transparent(BinaryColor::Off),
            &mut draw_target,
        )
        .unwrap();
    MEDIUM_SMALL_FONT
        .render_aligned(
            "Shapeshifter (Shark)",
            Point::new(0, 224),
            VerticalPosition::Baseline,
            HorizontalAlignment::Left,
            FontColor::Transparent(BinaryColor::Off),
            &mut draw_target,
        )
        .unwrap();


    SMALL_MONOSPACE_FONT
        .render_aligned(
            "bloup bloup",
            Point::new(64, 290),
            VerticalPosition::Baseline,
            HorizontalAlignment::Center,
            FontColor::Transparent(BinaryColor::Off),
            &mut draw_target,
        )
        .unwrap();


    draw_target.flush();
    info!("entering main loop");

    loop {
        info!("Display loop");
        // Off = black

        Timer::after_millis(300).await;
        // draw_target.clear(BinaryColor::On);
        // draw_target.flush();
    }
}

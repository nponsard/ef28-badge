use ariel_os::time::Timer;
use embedded_graphics::{
    Drawable as _, draw_target::DrawTarget as _, geometry::Point, image::Image,
    pixelcolor::BinaryColor,
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

use crate::{
    buttons::wait_for_button_released,
    drawer::DisplayTarget,
    screens::{BIG_FONT, MEDIUM_FONT, MEDIUM_SMALL_FONT, PFP, SMALL_MONOSPACE_FONT, Screen},
};

pub async fn presentation(draw_target: &mut DisplayTarget<'_>) -> Screen {
    draw_target.clear(BinaryColor::On);
    Image::new(&PFP, Point::zero()).draw(draw_target).unwrap();

    BIG_FONT
        .render_aligned(
            "sautax",
            Point::new(64, 160),
            VerticalPosition::Baseline,
            HorizontalAlignment::Center,
            FontColor::Transparent(BinaryColor::Off),
            draw_target,
        )
        .unwrap();

    MEDIUM_FONT
        .render_aligned(
            "They/Them",
            Point::new(64, 180),
            VerticalPosition::Baseline,
            HorizontalAlignment::Center,
            FontColor::Transparent(BinaryColor::Off),
            draw_target,
        )
        .unwrap();

    MEDIUM_SMALL_FONT
        .render_aligned(
            "OC: Emery",
            Point::new(0, 210),
            VerticalPosition::Baseline,
            HorizontalAlignment::Left,
            FontColor::Transparent(BinaryColor::Off),
            draw_target,
        )
        .unwrap();
    MEDIUM_SMALL_FONT
        .render_aligned(
            "Shapeshifter (Shark)",
            Point::new(0, 224),
            VerticalPosition::Baseline,
            HorizontalAlignment::Left,
            FontColor::Transparent(BinaryColor::Off),
            draw_target,
        )
        .unwrap();

    SMALL_MONOSPACE_FONT
        .render_aligned(
            "bloup bloup",
            Point::new(64, 290),
            VerticalPosition::Baseline,
            HorizontalAlignment::Center,
            FontColor::Transparent(BinaryColor::Off),
            draw_target,
        )
        .unwrap();

    draw_target.flush();

    wait_for_button_released().await;

    Screen::SettingsMain
}

use ariel_os::log::info;
use embedded_graphics::{
    Drawable as _,
    draw_target::DrawTarget as _,
    geometry::{Dimensions, Point},
    pixelcolor::BinaryColor,
    primitives::{Line, Primitive as _, PrimitiveStyle},
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

use crate::{
    buttons::{Button, wait_for_button_released},
    drawer::DisplayTarget,
    leds_controller::change_settings,
    screens::{MEDIUM_SMALL_FONT, MENU_FONT, Screen},
};

const HEADER_VERTICAL_PADDING: i32 = 30;

const LIST_ELEMENT_HEIGHT: i32 = 20;

fn draw_header(draw_target: &mut DisplayTarget<'_>, name: &str) {
    info!("header: {}", name);

    let width = draw_target.bounding_box().size.width;

    MEDIUM_SMALL_FONT
        .render_aligned(
            name,
            Point::new((width / 2) as i32, 10),
            VerticalPosition::Baseline,
            HorizontalAlignment::Center,
            FontColor::Transparent(BinaryColor::Off),
            draw_target,
        )
        .unwrap();

    info!("line: {}", name);

    Line::new(Point::new(10, 16), Point::new((width - 10) as i32, 16))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
        .draw(draw_target)
        .unwrap();
}

fn draw_list_element(draw_target: &mut DisplayTarget<'_>, name: &str, selected: u8, index: u8) {
    info!("list element: {}", name);
    const SELECT_PADDING: i32 = 5;

    let y = HEADER_VERTICAL_PADDING + i32::from(index) * LIST_ELEMENT_HEIGHT;
    MENU_FONT
        .render_aligned(
            name,
            Point::new(SELECT_PADDING, y),
            VerticalPosition::Baseline,
            HorizontalAlignment::Left,
            FontColor::Transparent(BinaryColor::Off),
            draw_target,
        )
        .unwrap();

    let width = draw_target.bounding_box().size.width;

    if selected == index {
        MENU_FONT
            .render_aligned(
                ">",
                Point::new(0, y),
                VerticalPosition::Baseline,
                HorizontalAlignment::Left,
                FontColor::Transparent(BinaryColor::Off),
                draw_target,
            )
            .unwrap();
    }
}

// - 0: Go to led settings
// - 1: Presentation screen
pub async fn settings_main(draw_target: &mut DisplayTarget<'_>) -> Screen {
    const ELEMENTS_COUNT: u8 = 2;
    let mut element_selected = 0;
    loop {
        draw_target.clear(BinaryColor::On);

        draw_header(draw_target, "Settings");

        draw_list_element(draw_target, "Led settings", element_selected, 0);
        draw_list_element(draw_target, "Presentation screen", element_selected, 1);

        draw_target.flush();

        match wait_for_button_released().await.button {
            Button::Left => element_selected = (element_selected + 1) % ELEMENTS_COUNT,
            Button::Right => match element_selected {
                0 => {
                    return Screen::SettingsLed;
                }
                1 => {
                    return Screen::Presentation;
                }
                _ => {
                    return Screen::Presentation;
                }
            },
        }
    }
}

// - 0: decrease brightness
// - 1: increase brightness
// - 2: go back to main settings
#[allow(clippy::unnecessary_min_or_max)]
pub async fn settings_led(draw_target: &mut DisplayTarget<'_>) -> Screen {
    const ELEMENTS_COUNT: u8 = 3;
    const MIN_BRIGHTNESS: u8 = 0;
    const MAX_BRIGHTNESS: u8 = 50;

    let mut element_selected = 0;
    loop {
        draw_target.clear(BinaryColor::On);

        draw_header(draw_target, "Settings");
        draw_list_element(draw_target, "Decrease brightness", element_selected, 0);
        draw_list_element(draw_target, "Increase brightness", element_selected, 1);
        draw_list_element(draw_target, "Back", element_selected, 2);

        draw_target.flush();

        match wait_for_button_released().await.button {
            Button::Left => element_selected = (element_selected + 1) % ELEMENTS_COUNT,
            Button::Right => match element_selected {
                0 => {
                    change_settings(|mut settings| {
                        settings.intensity =
                            MIN_BRIGHTNESS.max(settings.intensity.saturating_sub(1));
                        settings
                    })
                    .await;
                }
                1 => {
                    change_settings(|mut settings| {
                        settings.intensity =
                            MAX_BRIGHTNESS.min(settings.intensity.saturating_add(1));
                        settings
                    })
                    .await;
                }
                2 => {
                    return Screen::SettingsMain;
                }
                _ => {
                    return Screen::SettingsMain;
                }
            },
        }
    }
}

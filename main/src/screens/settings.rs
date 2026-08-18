use ariel_os::log::{debug, info};
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
    leds_controller::{change_led_settings, current_led_settings},
    screens::{MEDIUM_SMALL_FONT, MENU_FONT, Screen},
};

const HEADER_VERTICAL_PADDING: i32 = 30;

const LIST_ELEMENT_HEIGHT: i32 = 20;

fn draw_header(draw_target: &mut DisplayTarget<'_>, name: &str) {
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

    Line::new(Point::new(10, 16), Point::new((width - 10) as i32, 16))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
        .draw(draw_target)
        .unwrap();
}

fn list_element_y(index: u8) -> i32 {
    HEADER_VERTICAL_PADDING + i32::from(index) * LIST_ELEMENT_HEIGHT
}

fn draw_list_element(draw_target: &mut DisplayTarget<'_>, name: &str, selected: u8, index: u8) {
    debug!("list element: {}", name);
    const SELECT_PADDING: i32 = 5;

    let y = list_element_y(index);
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
// - 2: led mode submenu
// - 3: go back to main settings
#[allow(clippy::unnecessary_min_or_max)]
pub async fn settings_led(draw_target: &mut DisplayTarget<'_>) -> Screen {
    const ELEMENTS_COUNT: u8 = 4;
    const MIN_BRIGHTNESS: u8 = 0;
    const MAX_BRIGHTNESS: u8 = 50;

    let mut element_selected = 0;
    loop {
        draw_target.clear(BinaryColor::On);

        draw_header(draw_target, "Led Settings");
        draw_list_element(draw_target, "Decrease brightness", element_selected, 0);
        draw_list_element(draw_target, "Increase brightness", element_selected, 1);
        draw_list_element(draw_target, "Led mode", element_selected, 2);
        draw_list_element(draw_target, "Back", element_selected, 3);

        draw_target.flush();

        match wait_for_button_released().await.button {
            Button::Left => element_selected = (element_selected + 1) % ELEMENTS_COUNT,
            Button::Right => match element_selected {
                0 => {
                    let settings = change_led_settings(|mut settings| {
                        settings.intensity =
                            MIN_BRIGHTNESS.max(settings.intensity.saturating_sub(2));
                        info!("setting brightness to {}", settings.intensity);
                        settings
                    })
                    .await;
                    info!("setting brightness to {}", settings.intensity);
                }
                1 => {
                    let settings = change_led_settings(|mut settings| {
                        settings.intensity =
                            MAX_BRIGHTNESS.min(settings.intensity.saturating_add(2));
                        settings
                    })
                    .await;
                    info!("setting brightness to {}", settings.intensity);
                }
                2 => {
                    return Screen::SettingsLedMode;
                }
                3 => {
                    return Screen::SettingsMain;
                }
                _ => {
                    return Screen::SettingsMain;
                }
            },
        }
    }
}

#[allow(clippy::unnecessary_min_or_max)]
pub async fn settings_led_mode(draw_target: &mut DisplayTarget<'_>) -> Screen {
    const ELEMENTS_COUNT: u8 = 3;

    let width = draw_target.bounding_box().size.width as i32;

    let mut element_selected = 0;
    loop {
        let led_mode = current_led_settings().await.pattern;
        draw_target.clear(BinaryColor::On);

        draw_header(draw_target, "Led Mode");
        draw_list_element(draw_target, "Flag", element_selected, 0);
        draw_list_element(draw_target, "Breathing", element_selected, 1);
        draw_list_element(draw_target, "Back", element_selected, 2);

        MENU_FONT
            .render_aligned(
                "*",
                Point::new(width-10, list_element_y(led_mode )),
                VerticalPosition::Baseline,
                HorizontalAlignment::Right,
                FontColor::Transparent(BinaryColor::Off),
                draw_target,
            )
            .unwrap();

        draw_target.flush();

        match wait_for_button_released().await.button {
            Button::Left => element_selected = (element_selected + 1) % ELEMENTS_COUNT,
            Button::Right => match element_selected {
                // flag
                0 => {
                    change_led_settings(|mut settings| {
                        settings.pattern = 1;
                        settings
                    })
                    .await;
                }
                // Breathing
                1 => {
                    change_led_settings(|mut settings| {
                        settings.pattern = 2;
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

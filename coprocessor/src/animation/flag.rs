use crate::{
    ef_led::{LedDataArr, Rgb, LED_COUNT},
    led_data::LedData,
};
const BLACK: Rgb = (0, 0, 0);
const WHITE: Rgb = (255, 255, 255);
const PINK: Rgb = (230, 0, 100);
const LIGHT_BLUE: Rgb = (100, 100, 230);

const TRANS: LedDataArr = [
    BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, LIGHT_BLUE, LIGHT_BLUE, PINK, PINK, WHITE, WHITE,
    WHITE, PINK, PINK, LIGHT_BLUE, LIGHT_BLUE,
];

pub fn flag(settings: u16, tick: u16) -> (LedData, bool) {
    let selected_flag = settings as u8;

    let intensity = if tick < 150 { tick } else { (300 - tick) } + 105;

    let flag = match selected_flag {
        _ => TRANS,
    };

    let mut data = LedData::new(flag);

    data.apply_intensity(intensity as u8);

    (data, tick >= 300)
}

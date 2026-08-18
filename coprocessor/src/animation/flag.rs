use crate::{
    animation::colors::{BLACK, LIGHT_BLUE, PINK, RED, WHITE},
    ef_led::LedDataArr,
    led_data::LedData,
};

const TRANS: LedDataArr = [
    BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, LIGHT_BLUE, LIGHT_BLUE, PINK, PINK, WHITE, WHITE,
    WHITE, PINK, PINK, LIGHT_BLUE, LIGHT_BLUE,
];

const FRENCH: LedDataArr = [
    BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, RED, RED, RED, PINK, WHITE, WHITE, WHITE, PINK, PINK,
    LIGHT_BLUE, LIGHT_BLUE,
];

pub fn flag(settings: u16, tick: u16) -> (LedData, bool) {
    let selected_flag = settings as u8;

    let intensity = if tick < 150 { tick } else { 300 - tick } + 105;

    let flag = match selected_flag {
        _ => TRANS,
    };

    let mut data = LedData::new(flag);

    data.apply_intensity(intensity as u8);

    (data, tick >= 300)
}

use core::cmp::min;

use crate::{
    animation::colors::EMERY_TEAL,
    led_data::{apply_intensity_color, LedData},
};

const EYE_POSITION: usize = 2;

pub fn breathing(settings: u16, tick: u16) -> (LedData, bool) {
    const MAX: u16 = 320;
    const MID: u16 = MAX / 2;
    let selected_color = settings as u8;

    let color = match selected_color {
        0 => EMERY_TEAL,
        _ => EMERY_TEAL,
    };

    // From 0 to 160
    let step = if tick < MID { tick } else { MAX - tick };

    let intensity = min((step * step) / 100, 255);

    let mut data = LedData::default();
    data.set_individual(EYE_POSITION, apply_intensity_color(color, intensity as u8));

    data.apply_intensity(intensity as u8);

    (data, tick >= MAX)
}

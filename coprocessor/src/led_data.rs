use crate::ef_led::{LED_COUNT, LedDataArr, Rgb};

fn apply_intensity_element(value: u8, intensity: u8) -> u8 {
    ((value as u32 * 1000 * intensity as u32) / 255000) as u8
}

pub fn apply_intensity_iter<IT: Iterator<Item = Rgb>>(
    current_data: IT,
    intensity: u8,
) -> impl Iterator<Item = Rgb> {
    current_data.map(move |color| apply_intensity_color(color, intensity))
}

pub fn apply_intensity_color(color: Rgb, intensity: u8) -> Rgb {
    (
        apply_intensity_element(color.0, intensity),
        apply_intensity_element(color.1, intensity),
        apply_intensity_element(color.2, intensity),
    )
}

pub fn apply_intensity_mut(current_data: &mut LedDataArr, intensity: u8) {
    for color in current_data.iter_mut() {
        *color = apply_intensity_color(*color, intensity)
    }
}

#[derive(Default)]
pub struct LedData {
    inner: LedDataArr,
}

impl LedData {
    pub fn new(data: LedDataArr) -> Self {
        Self { inner: data }
    }
    pub fn fill(&mut self, col: Rgb) {
        self.inner = [col; LED_COUNT];
    }
    pub fn set_individual(&mut self, index: usize, col: Rgb) {
        if index < LED_COUNT {
            self.inner[index] = col
        }
    }
    pub fn replace(&mut self, data: LedDataArr) {
        self.inner = data;
    }
    pub fn into_iter(self) -> impl Iterator<Item = Rgb> {
        self.inner.into_iter()
    }

    pub fn apply_intensity(&mut self, intensity: u8) {
        apply_intensity_mut(&mut self.inner, intensity);
    }
}

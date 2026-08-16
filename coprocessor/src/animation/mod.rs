mod flag;

use esp_lp_hal::gpio::Output;

use crate::{
    animation::flag::flag,
    ef_led::{run, Rgb},
    led_data::{apply_intensity_iter, LedData},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pattern {
    Flag = 1,
}

impl From<u8> for Pattern {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Flag,
            _ => Self::Flag,
        }
    }
}

impl Into<u8> for Pattern {
    fn into(self) -> u8 {
        match self {
            Pattern::Flag => 1,
        }
    }
}

impl Pattern {
    // Returns led data and if the tick must be reset.
    pub fn run(&self, settings: u16, tick: u16) -> (LedData, bool) {
        match self {
            Self::Flag => flag(settings, tick),
        }
    }
}

pub struct Animator<const PIN: u8> {
    current_tick: u16,
    previous_pattern: Pattern,
    output: Output<PIN>,
}

impl<const PIN: u8> Animator<PIN> {
    pub fn new(output: Output<PIN>) -> Self {
        Self {
            current_tick: 0,
            previous_pattern: Pattern::Flag,
            output,
        }
    }

    pub fn tick(&mut self, intensity: u8, pattern: u8, pattern_settings: u16) {
        let new_pattern: Pattern = pattern.into();
        if self.previous_pattern != new_pattern {
            self.current_tick = 0;
            self.previous_pattern = new_pattern;
        }

        let (data, reset_tick) = self
            .previous_pattern
            .run(pattern_settings, self.current_tick);

        apply(&mut self.output, data.into_iter(), intensity);

        if reset_tick {
            self.current_tick = 0
        } else {
            self.current_tick = self.current_tick.wrapping_add(1);
        }
    }
}

fn apply<const PIN: u8>(output: &mut Output<PIN>, data: impl Iterator<Item = Rgb>, intensity: u8) {
    run(output, apply_intensity_iter(data, intensity));
}

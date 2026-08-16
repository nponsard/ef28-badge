//! # Buttons handling
//! ## Control scheme
//! - [`Left`] button: cycle options
//! - [`Right`] button: select option

use ariel_os::{
    gpio::{self, Level},
    log::info,
    time::{Duration, Instant},
};
use embassy_futures::select::Either;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

use crate::pins;

static BUTTON_CHANNEL: Channel<CriticalSectionRawMutex, Event, 10> = Channel::new();

const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(5);

pub enum Button {
    Left,
    Right,
}

pub struct Event {
    pub event_type: EventType,
    pub button: Button,
}
impl Event {
    pub fn new(button: Button, event_type: EventType) -> Self {
        Self { event_type, button }
    }
}

pub enum EventType {
    Released(Duration),
    Pressed,
}

#[ariel_os::task(autostart, peripherals)]
async fn button_handler(peripherals: pins::Buttons) {
    let mut left = gpio::Input::builder(peripherals.left_button, gpio::Pull::Up)
        .build_with_interrupt()
        .unwrap();
    let mut right = gpio::Input::builder(peripherals.right_button, gpio::Pull::Up)
        .build_with_interrupt()
        .unwrap();

    let mut last_left_released = Instant::now();
    let mut last_left_pressed = Instant::now();
    let mut last_right_released = Instant::now();
    let mut last_right_pressed = Instant::now();

    loop {
        match embassy_futures::select::select(left.wait_for_any_edge(), right.wait_for_any_edge())
            .await
        {
            Either::First(_) => {
                info!("Left button pressed");
                if left.get_level() == Level::High {
                    if last_left_pressed.elapsed() > DEBOUNCE_TIMEOUT {
                        let _ = BUTTON_CHANNEL.try_send(Event::new(
                            Button::Left,
                            EventType::Released(last_left_pressed.elapsed()),
                        ));
                    }
                    last_left_released = Instant::now();
                } else {
                    if last_left_released.elapsed() > DEBOUNCE_TIMEOUT {
                        let _ =
                            BUTTON_CHANNEL.try_send(Event::new(Button::Left, EventType::Pressed));
                    }
                    last_left_pressed = Instant::now();
                }
            }
            Either::Second(_) => {
                info!("Right button pressed");

                if right.get_level() == Level::High {
                    if last_right_pressed.elapsed() > DEBOUNCE_TIMEOUT {
                        let _ = BUTTON_CHANNEL.try_send(Event::new(
                            Button::Right,
                            EventType::Released(last_right_pressed.elapsed()),
                        ));
                    }
                    last_right_released = Instant::now();
                } else {
                    if last_right_released.elapsed() > DEBOUNCE_TIMEOUT {
                        let _ =
                            BUTTON_CHANNEL.try_send(Event::new(Button::Right, EventType::Pressed));
                    }
                    last_right_pressed = Instant::now();
                }
            }
        };
    }
}

pub async fn wait_for_button_event() -> Event {
    BUTTON_CHANNEL.receive().await
}

// Wait for a button to be released, discarding other events
pub async fn wait_for_button_released() -> Event {
    loop {
        let event = BUTTON_CHANNEL.receive().await;
        if matches!(event.event_type, EventType::Released(_)) {
            return event;
        }
    }
}

// Wait for a button to be pressed, discarding other events
pub async fn wait_for_button_pressed() -> Event {
    loop {
        let event = BUTTON_CHANNEL.receive().await;
        if matches!(event.event_type, EventType::Pressed) {
            return event;
        }
    }
}

pub fn clear_inputs() {
    BUTTON_CHANNEL.clear();
}

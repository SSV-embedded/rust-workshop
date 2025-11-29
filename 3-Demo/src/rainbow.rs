//! # Rainbow Animation Driver
//!
//! Displays a rainbow effect on a given [crate::led::Led] instance.
//!
//! It exposes a [HueReporter] which outputs the current LED's hue every second and a [HueAdjuster] to adjust the current animation hue.

use crate::led::Led;
use core::num::Wrapping;
use embassy_executor::Spawner;
use embassy_futures::select::{Either3, select3};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, Receiver, Sender},
};
use embassy_time::{Duration, Ticker};
use static_cell::StaticCell;

const DISPLAY_INTERVAL: Duration = Duration::from_millis(1000 / 25);
const UPDATE_INTERVAL: Duration = Duration::from_millis(1000);
const SPEED: u16 = 25;
const ADJUSTMENT_DIVISOR: u16 = 16;
const MAX_ACCELERATION: u16 = 128;

#[embassy_executor::task]
async fn rainbow_animation_task(
    led: &'static mut Led,
    hue_updates_out: &'static Channel<CriticalSectionRawMutex, u16, 1>,
    hue_updates_in: &'static Channel<CriticalSectionRawMutex, u16, 1>,
) {
    let mut hue = Wrapping(0u16);
    let mut display_tick = Ticker::every(DISPLAY_INTERVAL);
    let mut update_tick = Ticker::every(UPDATE_INTERVAL);
    loop {
        match select3(
            display_tick.next(),
            update_tick.next(),
            hue_updates_in.receive(),
        )
        .await
        {
            Either3::First(_) => {
                led.set_hue(hue.0).await;
                hue += SPEED;
            }
            Either3::Second(_) => {
                let _ = hue_updates_out.try_send(hue.0);
            }
            Either3::Third(other_hue) => {
                let diff = Wrapping(other_hue) - hue;
                if diff.0 <= u16::MAX / 2 {
                    let speedup = diff.0 / ADJUSTMENT_DIVISOR;
                    let speedup = speedup.min(MAX_ACCELERATION);
                    defmt::info!(
                        "our hue: {} - other hue: {} - speedup: {}",
                        hue.0,
                        other_hue,
                        speedup
                    );
                    hue += speedup;
                } else {
                    let slowdown = (u16::MAX - diff.0) / ADJUSTMENT_DIVISOR;
                    let slowdown = slowdown.min(MAX_ACCELERATION);
                    defmt::info!(
                        "our hue: {} - other hue: {} - slowdown: {}",
                        hue.0,
                        other_hue,
                        slowdown
                    );
                    hue -= slowdown;
                };
            }
        }
    }
}

/// Animation hue adjuster
pub struct HueAdjuster {
    ch: Sender<'static, CriticalSectionRawMutex, u16, 1>,
}

impl HueAdjuster {
    fn new(ch: Sender<'static, CriticalSectionRawMutex, u16, 1>) -> Self {
        Self { ch }
    }

    /// Adjust the current hue in the direction of `other_hue`.
    pub async fn adjust(&self, other_hue: u16) {
        self.ch.send(other_hue).await;
    }
}

/// Animation hue reporter
pub struct HueReporter {
    ch: Receiver<'static, CriticalSectionRawMutex, u16, 1>,
}

impl HueReporter {
    fn new(ch: Receiver<'static, CriticalSectionRawMutex, u16, 1>) -> Self {
        Self { ch }
    }

    /// Receives the current hue every second.
    pub async fn recv(&self) -> u16 {
        self.ch.receive().await
    }
}

/// Starts a rainbow animation on `led`.
pub fn start_animation(spawner: &Spawner, led: Led) -> (HueReporter, HueAdjuster) {
    let led = {
        static CELL: StaticCell<Led> = StaticCell::new();
        CELL.init(led)
    };
    let hue_updates_out = {
        static CELL: StaticCell<Channel<CriticalSectionRawMutex, u16, 1>> = StaticCell::new();
        CELL.init(Channel::new())
    };
    let hue_updates_in = {
        static CELL: StaticCell<Channel<CriticalSectionRawMutex, u16, 1>> = StaticCell::new();
        CELL.init(Channel::new())
    };

    spawner.must_spawn(rainbow_animation_task(led, hue_updates_out, hue_updates_in));

    (
        HueReporter::new(hue_updates_out.receiver()),
        HueAdjuster::new(hue_updates_in.sender()),
    )
}

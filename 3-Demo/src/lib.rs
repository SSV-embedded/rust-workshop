#![no_std]

extern crate alloc;

pub mod led;
pub mod net;
pub mod rainbow;
pub mod rtos;

/* === YOUR CHANGES DOWN BELOW! DON'T TOUCH THE CODE ABOVE! === */

use embassy_futures::select::{Either, select};

/// Application ... this is your playground!
pub async fn main(spawner: embassy_executor::Spawner, peripherals: esp_hal::peripherals::Peripherals) {
    // Start RTOS
    rtos::start(peripherals.TIMG0, peripherals.SW_INTERRUPT);

    // Start network stack
    let key = b"CHANGEMECHANGEMECHANGEMECHANGEME";
    let (net_rx, net_tx) = net::start_net::<u16>(&spawner, peripherals.WIFI, &key);

    // Start animation
    let led = led::Led::new(peripherals.SPI2, peripherals.GPIO8);
    let (hue_reporter, hue_adjuster) = rainbow::start_animation(&spawner, led);

    loop {
        match select(net_rx.recv(), hue_reporter.recv()).await {
            Either::First(hue) => {
                hue_adjuster.adjust(hue).await;
            }
            Either::Second(hue) => {
                net_tx.send(hue).await;
            }
        }
    }
}

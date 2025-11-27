#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use esp_hal::clock::CpuClock;
use panic_rtt_target as _;
use rust_demo::{led::Led, net::start_net, rainbow::start_animation, start_rtos};

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    rtt_target::rtt_init_defmt!();

    // Init SoC
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Start RTOS
    start_rtos(peripherals.TIMG0, peripherals.SW_INTERRUPT);

    // Start network stack
    let key = b"CHANGEMECHANGEMECHANGEMECHANGEME";
    let (net_rx, net_tx) = start_net::<u16>(&spawner, peripherals.WIFI, &key);

    // Start animation
    let led = Led::new(peripherals.SPI2, peripherals.GPIO8);
    let (hue_reporter, hue_adjuster) = start_animation(&spawner, led);

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

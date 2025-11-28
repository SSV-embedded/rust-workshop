#![no_std]
#![no_main]

extern crate alloc;

use embassy_executor::Spawner;
use panic_rtt_target as _;
use rtt_target::rtt_init_defmt;
use esp_hal::{Config, clock, init};

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    rtt_init_defmt!();

    // Init SoC
    let config = Config::default().with_cpu_clock(clock::CpuClock::max());
    let peripherals = init(config);

    rust_demo::main(spawner, peripherals).await;

    loop {}
}

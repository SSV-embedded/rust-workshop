#![no_std]

extern crate alloc;

pub mod led;
pub mod net;
pub mod rainbow;
pub mod rtos;

use embassy_executor::Spawner;
use esp_hal::peripherals::Peripherals;

/* === YOUR CHANGES DOWN BELOW! DON'T TOUCH THE CODE ABOVE! === */

/// Application ... this is your playground!
pub async fn main(spawner: Spawner, peripherals: Peripherals) {
    defmt::info!("Hallo Workshop!");
}

//! # RGB LED Driver
//! 
//! Example for running the LED on the ESP32-C6-MINI board
//! 
//! ```rust
//! use embassy_time::{Timer, Duration};
//! 
//! #[esp_rtos::main]
//! async fn main(spawner: embassy_executor::Spawner) -> ! {
//!     // Init debug outputs via RTT
//!     rtt_target::rtt_init_defmt!();
//!     
//!     // Init SoC
//!     let config = esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max());
//!     let peripherals = esp_hal::init(config);
//! 
//!     // Start RTOS for time driver
//!     rust_demo::start_rtos(peripherals.TIMG0, peripherals.SW_INTERRUPT);
//! 
//!     // Init the LED on GPIO 8 driven by the SPI2 peripheral
//!     let led = rust_demo::led::Led::new(peripherals.SPI2, peripherals.GPIO8);
//! 
//!     loop {
//!         // Set the color to red
//!         led.set_hue(0).await;
//!         defmt::info!("RED");
//! 
//!         // Wait 1s
//!         Timer::after(Duration::from_secs(1)).await;
//!         
//!         // Set the color to green
//!         led.set_hue(u16::MAX / 3).await;
//!         defmt::info!("GREEN");
//! 
//!         // Wait 1s
//!         Timer::after(Duration::from_secs(1)).await;
//! 
//!         // Set the color to blue
//!         led.set_hue(u16::MAX / 3 * 2).await;
//!         defmt::info!("BLUE");
//! 
//!         // Wait 1s
//!         Timer::after(Duration::from_secs(1)).await;
//!     }
//! }
//! ```

use esp_hal::{
    Async,
    gpio::interconnect::PeripheralOutput,
    spi::master::{Config, Instance, Spi},
    time::Rate,
};
use smart_leds::{
    RGB, SmartLedsWriteAsync,
    hsv::{Hsv, hsv2rgb},
};
use static_cell::StaticCell;
use ws2812_async::{Rgb, Ws2812};

pub struct Led(Ws2812<Spi<'static, Async>, Rgb, 12>);

impl Led {
    /// Initializes a new RGB LED instance
    pub fn new(
        spi: impl Instance + 'static,
        pin: impl PeripheralOutput<'static>,
    ) -> &'static mut Self {
        let config = Config::default().with_frequency(Rate::from_hz(3_800_000));
        let spi = Spi::new(spi, config).unwrap().with_mosi(pin).into_async();
        let led = Ws2812::<_, Rgb, 12>::new(spi);
        static CELL: StaticCell<Led> = StaticCell::new();
        CELL.init(Self(led))
    }

    /// Sets the LED's hue.
    /// 
    /// `0` -> red, `u16::MAX / 3` -> green, `u16::MAX / 3 * 2` -> blue
    pub async fn set_hue(&mut self, hue: u16) {
        let hue = (hue / 256) as u8;
        let color = hsv2rgb(Hsv {
            hue,
            sat: 255,
            val: 255,
        });
        self.0.write([color]).await.unwrap();
    }

    /// Turns the LED off
    pub async fn clear(&mut self) {
        self.0.write([RGB::new(0, 0, 0)]).await.unwrap();
    }
}

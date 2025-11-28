use esp_alloc::heap_allocator;
use esp_hal::{
    interrupt::software::SoftwareInterruptControl,
    peripherals::{SW_INTERRUPT, TIMG0},
    timer::timg::TimerGroup,
};

/// Helper for starting the ESP RTOS for global timing and the network stack.
pub fn start(timer: TIMG0<'static>, sw_int: SW_INTERRUPT<'static>) {
    // Prepare heap memeory
    heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);

    // Start RTOS for WIFI stack
    let timg0 = TimerGroup::new(timer);
    let sw_interrupt = SoftwareInterruptControl::new(sw_int);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);
}

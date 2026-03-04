#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // Initialize RGB LED pins (active low)
    let mut red = Output::new(p.P0_26, Level::High, OutputDrive::Standard);
    let mut green = Output::new(p.P0_30, Level::High, OutputDrive::Standard);
    let mut blue = Output::new(p.P0_06, Level::High, OutputDrive::Standard);

    info!("Starting RGB LED pattern...");

    loop {
        info!("Simple RGB cycle");

        // Red on
        red.set_low();
        green.set_high();
        blue.set_high();
        Timer::after_millis(1000).await;

        // Green on
        red.set_high();
        green.set_low();
        blue.set_high();
        Timer::after_millis(1000).await;

        // Blue on
        red.set_high();
        green.set_high();
        blue.set_low();
        Timer::after_millis(1000).await;
    }
}

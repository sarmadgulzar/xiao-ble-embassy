#![no_std]
#![no_main]

use defmt::info;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_nrf::init(Default::default());

    // Initialize RGB LED pins (active low)
    info!("Initializing the onboard Red LED...");
    let mut onboard_red = Output::new(p.P0_26, Level::High, OutputDrive::Standard);

    info!("Initializing the button...");
    let button = Input::new(p.P0_03, Pull::Up);

    loop {
        if button.is_low() {
            info!("Pressed...");
            onboard_red.set_low();
        } else {
            info!("Not Pressed...");
            onboard_red.set_high();
        }
    }
}

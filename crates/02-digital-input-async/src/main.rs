#![no_std]
#![no_main]
use defmt::info;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
// use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_nrf::init(Default::default());

    info!("Initializing the onboard Red LED...");
    let mut onboard_red = Output::new(p.P0_26, Level::High, OutputDrive::Standard);

    // info!("Initializing the heartbeat LED...");
    // let mut heartbeat = Output::new(p.P0_02, Level::Low, OutputDrive::Standard);

    info!("Initializing the button...");
    let mut button = Input::new(p.P0_03, Pull::Up);

    loop {
        button.wait_for_low().await;

        onboard_red.set_low();
        info!("Button pressed!");

        // heartbeat.toggle();
        // info!("Heartbeat: {}", heartbeat.get_output_level());
        // Timer::after_millis(500).await;

        button.wait_for_high().await;
        onboard_red.set_high();
    }
}

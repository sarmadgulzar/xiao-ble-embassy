#![no_std]
#![no_main]
use defmt::{info, unwrap};
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn button_task(mut button: Input<'static>, mut red_led: Output<'static>) {
    loop {
        button.wait_for_low().await;
        red_led.set_low(); // Red LED ON
        info!("Button pressed!");

        button.wait_for_high().await;
        red_led.set_high(); // Red LED OFF
        info!("Button released!");
    }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_nrf::init(Default::default());

    info!("Initializing the onboard Red LED...");
    let onboard_red = Output::new(p.P0_26, Level::High, OutputDrive::Standard);

    info!("Initializing the heartbeat LED...");
    let mut heartbeat = Output::new(p.P0_02, Level::Low, OutputDrive::Standard);

    info!("Initializing the button...");
    let button = Input::new(p.P0_03, Pull::Up);

    // Move ownership of the button and red LED into another task.
    spawner.spawn(unwrap!(button_task(button, onboard_red)));

    loop {
        heartbeat.toggle();
        info!("Heartbeat: {}", heartbeat.get_output_level());
        Timer::after_millis(1000).await;
    }
}

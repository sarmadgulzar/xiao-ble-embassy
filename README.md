# XIAO BLE Embassy

Embassy-based Rust examples for XIAO BLE boards. This repository uses a workspace layout under `crates/` and provides a repeatable pattern for creating and running new firmware crates.

## Start a new crate

Create a new crate inside `crates/` and set the crate name:

```bash
cargo new crates/07-cool-project --name cool-project
```

## Update `Cargo.toml`

Replace `crates/07-cool-project/Cargo.toml` with:

```toml
[package]
name = "cool-project"
edition.workspace = true
license.workspace = true
version.workspace = true
publish.workspace = true
build = "../../build.rs"

[dependencies]
embassy-executor.workspace = true
embassy-time.workspace = true
embassy-nrf.workspace = true

defmt.workspace = true
defmt-rtt.workspace = true
panic-probe.workspace = true

cortex-m.workspace = true
cortex-m-rt.workspace = true
```

## Update `src/main.rs`

Replace `crates/07-cool-project/src/main.rs` with:

```rust
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
```

## Run the new crate

From the repository root, run:

```bash
cargo run --bin cool-project
```

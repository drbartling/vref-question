#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_time::{Duration, Timer};
use fmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut adc = Adc::new(p.ADC1);
    let mut vref = adc.enable_vrefint();
    let vref_counts = adc.blocking_read(&mut vref);
    info!("vref_counts: {:?}", vref_counts);

    loop {
        Timer::after(Duration::from_secs(86400)).await;
    }
}

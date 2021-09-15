#![no_main]
#![no_std]

mod lib;

use cortex_m::asm::delay;
use sensor_data::{entry, init, iprintln};
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use crate::lib::init;

/// This project pertains to
#[entry]
fn main() -> ! {
    let (mut lsm303agr, mut itm) = init();

    loop {
        let magnetometer_data = lsm303agr
            .mag_data()
            .expect("Unable to read magnetometer's data");
        iprintln!(
            &mut itm.stim[0],
            "{{ x: {:?}, y: {:?} , z: {:?} }}",
            magnetometer_data.x,
            magnetometer_data.y,
            magnetometer_data.z
        );
        delay(20_000_000_u32);
    }
}

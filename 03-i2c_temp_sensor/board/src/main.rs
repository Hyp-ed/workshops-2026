#![no_std]
#![no_main]

use core::panic;
use embassy_executor::Spawner;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_time::{Duration, Timer};
use hyped_io_stm32l476rg::i2c::Stm32l476rgI2c;
use hyped_sensors::temperature::{Status, Temperature};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());
    let hyped_i2c = Stm32l476rgI2c::new(i2c);

    // Task 2: Create a new instance of the temperature sensor

    let mut temperature_sensor = todo!();

    // END OF TASK 2

    loop {
        match temperature_sensor.check_status() {
            Status::TempOverUpperLimit => {
                defmt::error!("Temperature is over the upper limit.");
            }
            Status::TempUnderLowerLimit => {
                defmt::error!("Temperature is under the lower limit.");
            }
            Status::Busy => {
                defmt::warn!("Temperature sensor is busy.");
            }
            Status::Unknown => {
                panic!("Could not get the status of the temperature sensor.")
            }
            Status::Ok => {}
        }

        // TASK 3: Call the read method of the temperature sensor and print the temperature



        // END OF TASK 3
    }
}

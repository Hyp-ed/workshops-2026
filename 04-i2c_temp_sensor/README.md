# Sensors Workshop - I2C Temperature Sensor

This workshop will guide you through the process of reading temperature data from an I2C temperature sensor using an STM32 microcontroller. The workshop is divided into the following sections:

1. [Introduction to I2C](#introduction-to-i2c)
2. [Hardware Required](#hardware-required)
3. [Hardware Setup](#hardware-setup)
4. [Software Prerequisites](#software-prerequisites)
5. [Fill in the Blanks](#fill-in-the-blanks)
6. [Run it!](#run-it)

## Introduction to I2C

I2C (Inter-Integrated Circuit) is a serial communication protocol that allows multiple devices to communicate with each other using only two wires. The I2C bus consists of two lines: SDA (Serial Data) and SCL (Serial Clock). The SDA line is used to transfer data between devices, while the SCL line is used to synchronize the data transfer.

We have provided you with our `HypedI2C` abstraction layer which simplifies the process of reading data from I2C devices. The `HypedI2C` struct provides functions to read and write data to I2C devices, and is part of our `hyped_io` crate library that will eventually contain similar abstractions for other I/O protocols.

## Hardware Required

- STM32L476RG Nucleo-32 development board
- I2C temperature sensor (STTS22H) (you may have to share this with other participants - but you only need to have the sensor when you are testing your completed code)

The datasheet for the STTS22H temperature sensor can be found [here](https://www.st.com/resource/en/datasheet/stts22h.pdf). You will need to refer to this datasheet in order to work out how to read the temperature data from the sensor!

## Hardware Setup

1. Connect the STM32L476RG Nucleo-32 development board to your computer using a USB cable.
2. Connect the I2C temperature sensor to the Nucleo-32 development board as follows:
   - Sensor VIN to Nucleo-32 3.3V
   - Sensor GND to Nucleo-32 GND
   - Sensor SDA to Nucleo-32 D14
   - Sensor SCL to Nucleo-32 D15

## Software Prerequisites

Make sure that you have Rust and `probe-rs` installed on your computer. You can find the instructions on our [Getting Started with Rust Wiki page](https://github.com/Hyp-ed/hyped-2025/wiki/Getting-Started-with-Rust).

## Fill in the Blanks!

In this section, you will complete the code provided so that the STM32 microcontroller reads temperature data from the I2C temperature sensor. The code is divided into three parts:

1. **Temperature reading logic**: In the `sensors/src/temperature/rs` file, you need to implement the `read` function that reads temperature data from the I2C temperature sensor. You can use the `HypedI2C` struct provided to communicate with the sensor, you can see the methods available in the `io/src/i2c.rs` file. (You will need to read the datasheet for the sensor in order to work out how to read the temperature data.) All of the required register addresses are provided in the file, and may give you a hint as to how to read the temperature data. Make sure that you return the temperature data in degrees Celsius!

2. **Initialise the temperature sensor**: In the `board/src/mainr.rs` file, you need to write the code to create a new `Temperature` struct. Have a look at the parameters of the `Temperature::new` function in the `sensors/src/temperature.rs` file to see what you need to pass in.

3. **Print the temperature**: In the `board/src/main.rs` file, you need to write the code to print the temperature data to the console. You can use the `dfmt::info!` macro to print the temperature data. Look at the `read` function that you implemented earlier to see how to get the temperature data.

## Run it!

Run the code on your STM32L476RG Nucleo-32 development board and check the console output to see the temperature data. To run the code, you can use the following command:

```bash
cargo run
```

If you have completed all the steps correctly, you should see the temperature data printed to the console. Congratulations! You have successfully read temperature data from an I2C temperature sensor using an STM32 microcontroller.

# Rust + STM Blinky Lights Workshop

This workshop will guide you through the process of setting up a Rust project for an STM32 microcontroller, and writing a simple program that blinks an LED.

_We are assuming that you have already installed Rust and the necessary tools. If you haven't, please refer to the [Getting Started with Rust](https://github.com/Hyp-ed/hyped-2025/wiki/Getting-Started-with-Rust) Wiki page._

## Setup

### Create a file in `src/bin`

Call it whatever you like, it could be your name or something funny. Everyone will write their code in their own file so that we can merge everyone's code into the GitHub repo at the end of the workshop to test your git knowledge!

The barebones code for running Rust on an STM32 microcontroller is as follows:

```rust
#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // CODE HERE
}
```

You can see an example of this code in the `src/bin/example.rs`.

### Initialising

```rust
  let p = embassy_stm32::init(Default::default());
```

### The LED

```rust
    let mut led = Output::new(p.PA5, Level::High, Speed::Low);
```

This creates a new `Output` struct, which represents a GPIO pin that can be written to. The first argument is the pin, the second is the initial state of the pin, and the third is the speed of the pin.

On this model of STM32, the LED is connected to pin PB7. This means that if we set this pin high, the LED will turn on.

This will set the LED to high, turning it on:

```rust
    led.set_high();
```

Conversely, this will set the LED to low, turning it off:

```rust
    led.set_low();
```

### The Loop

All code written for the STM microcontrollers have a loop that runs forever. This is because the microcontroller is designed to run a single program forever, and so we must write our code to run in a loop.

```rust
    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
```

This code turns the LED on, waits for 300 milliseconds, turns the LED off, waits for 300 milliseconds, and then repeats.

### Running the Code

To run the code, you can simply run `cargo run --bin <your file name>`. This will compile the code and flash it to the microcontroller. If you have set everything up correctly, the LED should start blinking!

### Make it unique!

Try changing the speed of the LED, or the time it is on and off for.

## Test your git knowledge!

Once you are happy with your code and have tested it on the microcontroller, you can push your code to the GitHub repo. This will test your git knowledge and allow us to see everyone's code!

To do this, you will need to create a new branch, commit your code, and push it to the repo. To recap, you can use the following commands:

```bash
git checkout -b <your branch name>
git add .
git commit -m "Add my code"
git push origin <your branch name>
```

Please conform to the following naming convention for your branch: `03-rust-blinky-lights/<your name>`. For example, if your name is Ferris, your branch name would be `03-rust-blinky-lights/ferris`.

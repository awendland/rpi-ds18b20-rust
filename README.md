# RPI DS18B20 Temp Sensor

This is a toy program written in [Rust](https://www.rust-lang.org/) for retrieving temperature data from a [DS18B20 sensor](https://www.sunfounder.com/learn/sensor-kit-v2-0-for-raspberry-pi-b-plus/lesson-26-ds18b20-temperature-sensor-sensor-kit-v2-0-for-b-plus.html) attached via the [one-wire interface](https://pinout.xyz/pinout/1_wire#) to a [Raspberry Pi](https://www.raspberrypi.org/).

## Setup

This was developed on macOS and cross-compiled for a Raspberry Pi 3. To ease in cross-compiling, [cross](https://github.com/rust-embedded/cross) and [rustup](https://rustup.rs/) were used.

First, `rustup` was used to retrieve the appropriate target components `rustup target add armv7-unknown-linux-musleabihf`.

Then, `cross` was used to transparently create a Docker environment for the cross-compilation toolchain via `cross build --target armv7-unknown-linux-musleabihf`. Output from `cross` is placed in `target/` as if `cargo` was used directly.

## Usage

The program is quite simple (it mostly just reads from the `w1` virtual file system). By default it outputs the temperature from the sensor in farenheit, or if either `-r/--raw` is passed it outputs it in millicelsius.

## Expanded Usage

I setup a cron job on my Raspberry Pi to submit the latest temperature data to a Google Sheet via one of the proxy Google Sheet API services. For example, the cron entry looks something like:

```
*/15 18,19,20 * * * curl -X POST 'https://api.steinhq.com/v1/storages/TOKEN/SHEET' --data "[{ \"Time\": \"$(date -u +"\%Y-\%m-\%dT\%H:\%M:\%SZ")\", \"Temperature\": $(~/rpi-ds18b20-rust --raw) }]"
```


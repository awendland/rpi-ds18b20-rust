mod ds18b20;
mod w1_errors;
use std::env;

fn main() {
    let sensor = ds18b20::DS18B20::new().unwrap();
    let temp = sensor.read_temp().unwrap();

    // Default print friendly, but allow "-r/--raw" to
    // be passed to display the millicelsius directly instead
    let args: Vec<String> = env::args().collect();
    if (args.len() > 1) && args[1].contains("-r") {
        println!("{}", temp.as_u32());
    }
    else {
        println!("{:.1} F", temp.to_fahrenheit());
    }
}
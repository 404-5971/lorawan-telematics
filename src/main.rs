use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys;
use log::info;
use std::error::Error;
use std::time::Duration;

mod deep_sleep;
use deep_sleep::enter_deep_sleep;

mod gps;
use gps::get_lat_lon;

fn main() -> Result<(), Box<dyn Error>> {
    sys::link_patches();
    EspLogger::initialize_default();

    // 1. Take peripherals ONCE at the top of main
    let peripherals = Peripherals::take()?;
    let pins = peripherals.pins;

    // 2. Perform your logic
    // Give USB some time to enumerate (Crucial for seeing logs after a wake-up reboot)
    std::thread::sleep(Duration::from_secs(3));
    info!("Device is awake (Booted/Reset)! Waiting for USB enumeration...");

    info!("Initializing GPIOs...");
    unsafe {
        sys::gpio_hold_dis(pins.gpio3.pin());
        sys::gpio_hold_dis(pins.gpio2.pin());
    }

    let (gps_fix, gpio3) = get_lat_lon(peripherals.uart1, pins.gpio33, pins.gpio34, pins.gpio3)?;
    match gps_fix {
        Some((lat, lon)) => info!("Lat: {}, Lon: {}", lat, lon),
        None => info!("No GPS fix found"),
    }

    for i in 0..5 {
        info!("Deep sleeping in {} seconds...", 5 - i);
        std::thread::sleep(Duration::from_secs(1));
    }

    info!("Deep sleeping now...");
    // 3. Prepare for sleep
    enter_deep_sleep(gpio3, pins.gpio2)

    // Everything after this point it unreachable because enter_deep_sleep resets the device
}

use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::sys;
use log::info;
use std::error::Error;
use std::time::Duration;

pub fn enter_deep_sleep(gpio3: Gpio3, gpio2: Gpio2) -> Result<(), Box<dyn Error>> {
    // 1. Configure Pins
    let mut vext = PinDriver::output(gpio3)?;
    let mut adc_ctrl = PinDriver::output(gpio2)?;

    vext.set_low()?;
    adc_ctrl.set_low()?;

    // 2. Enable Hold (Still requires unsafe for the raw sys call)
    // There is no standard "safe" HAL wrapper for hold_en yet that covers all chips perfectly.
    unsafe {
        sys::gpio_hold_en(vext.pin());
        sys::gpio_hold_en(adc_ctrl.pin());
        // Required for ESP32-S3 to maintain digital GPIO state in deep sleep
        sys::gpio_deep_sleep_hold_en();
    }

    // 3. Configure Sleep
    // We can use the safe wrapper for sleeping if we want,
    // though 'esp_deep_sleep_start' is often preferred for clarity on what exactly is happening.
    let sleep_duration = Duration::from_secs(10);
    info!("Entering Deep Sleep for {:?}...", sleep_duration);

    unsafe {
        sys::esp_sleep_enable_timer_wakeup(sleep_duration.as_micros() as u64);

        // Free memory/resources before hard shutdown (optional but good practice)
        // Explicitly drop drivers (though memory is wiped on sleep anyway)
        drop(vext);
        drop(adc_ctrl);

        sys::esp_deep_sleep_start();
    }
}

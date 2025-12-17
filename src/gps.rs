use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{Gpio3, Gpio33, Gpio34, PinDriver};
use esp_idf_svc::hal::uart::{UartConfig, UartDriver, UART1};
use esp_idf_svc::hal::units::Hertz;
use log::info;
use nmea::Nmea;
use std::error::Error;
use std::time::Duration;

type GpsFix = Option<(f64, f64)>;

pub fn get_lat_lon(
    uart: UART1,
    tx: Gpio33,
    rx: Gpio34,
    mut power_pin: Gpio3,
) -> Result<(GpsFix, Gpio3), Box<dyn Error>> {
    // 1. Hardware Enable (GPIO 3)
    // The V1.2 requires GPIO 3 High to power the GNSS module.
    let mut gps_power = PinDriver::output(&mut power_pin)?;
    gps_power.set_high()?;
    println!("GPS Power Enabled via GPIO 3");

    // Give the module a moment to wake up
    FreeRtos::delay_ms(500);

    // 2. UART Configuration
    // TX: GPIO 33, RX: GPIO 34
    let config = UartConfig::new().baudrate(Hertz(115200));
    let uart = UartDriver::new(
        uart,
        tx,
        rx,
        Option::<esp_idf_svc::hal::gpio::Gpio0>::None, // No CTS
        Option::<esp_idf_svc::hal::gpio::Gpio0>::None, // No RTS
        &config,
    )?;

    // 3. Parser Loop
    let mut nmea = Nmea::default();
    let mut buffer = [0u8; 1]; // Read byte by byte for simplicity

    // Create a vector to hold the current line
    let mut line_buffer = String::new();

    info!("Waiting for GPS fix...");

    for _ in 0..3 {
        // Read 1 byte with a timeout
        match uart.read(&mut buffer, 100) {
            Ok(bytes_read) if bytes_read > 0 => {
                let char = buffer[0] as char;

                if char == '\n' {
                    // Line complete, attempt parse
                    if let Ok(_msg) = nmea.parse(&line_buffer) {
                        if let (Some(lat), Some(lon)) = (nmea.latitude, nmea.longitude) {
                            // Return the pin back so it can be used for deep sleep hold
                            drop(gps_power);
                            let power_pin = power_pin;
                            return Ok((Some((lat, lon)), power_pin));
                        }
                    }
                    line_buffer.clear();
                } else {
                    line_buffer.push(char);
                }
            }
            _ => {
                // Timeout or no data, just continue
                // Ideally, sleep briefly to yield if no data
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }
    drop(gps_power);
    let power_pin = power_pin;
    Ok((None, power_pin))
}

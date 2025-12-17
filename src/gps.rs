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
    rx: Gpio33,
    tx: Gpio34,
    mut power_pin: Gpio3,
) -> Result<(GpsFix, Gpio3), Box<dyn Error>> {
    // 1. Hardware Enable (GPIO 3)
    let mut gps_power = PinDriver::output(&mut power_pin)?;
    gps_power.set_high()?;
    info!("GPS Power Enabled via GPIO 3");

    // Give the module a moment to wake up
    FreeRtos::delay_ms(500);

    // 2. UART Configuration
    let config = UartConfig::new().baudrate(Hertz(115200));
    let uart = UartDriver::new(
        uart,
        tx,
        rx,
        Option::<esp_idf_svc::hal::gpio::Gpio0>::None,
        Option::<esp_idf_svc::hal::gpio::Gpio0>::None,
        &config,
    )?;

    // 3. Send Initialization Command (From C++ Reference)
    // This configures the system (CFGSYS) for the specific Air530/UC6580 mode
    let init_cmd = b"$CFGSYS,h35155*68\r\n";
    uart.write(init_cmd)?;
    info!("Sent GNSS Init Command");

    // 4. Parser Loop
    let mut nmea = Nmea::default();
    let mut buffer = [0u8; 256];

    // IMPORTANT: Buffer must be OUTSIDE the loop to accumulate fragments
    let mut sentence_accumulator = String::new();

    info!("Waiting for GPS fix...");
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < Duration::from_secs(5 * 60) {
        match uart.read(&mut buffer, 100) {
            Ok(bytes_read) if bytes_read > 0 => {
                // Iterate over received bytes
                for &byte in &buffer[..bytes_read] {
                    let char = byte as char;
                    sentence_accumulator.push(char);

                    // Check for end of line (NMEA standard ends with \r\n)
                    if char == '\n' {
                        let line = sentence_accumulator.trim();

                        match nmea.parse(line) {
                            Ok(_) => {
                                if let (Some(lat), Some(lon)) = (nmea.latitude, nmea.longitude) {
                                    drop(gps_power);
                                    return Ok((Some((lat, lon)), power_pin));
                                }
                            }
                            Err(_e) => {}
                        }
                        sentence_accumulator.clear();
                    }
                }
            }
            _ => {
                FreeRtos::delay_ms(10);
            }
        }
    }

    info!("GPS Timeout");
    drop(gps_power);
    Ok((None, power_pin))
}

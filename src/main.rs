use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::Level;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::gpio::Pull;
use esp_idf_hal::peripherals::Peripherals;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    let peripherals = Peripherals::take().unwrap();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    let mut pin = PinDriver::output(peripherals.pins.gpio32);
    loop {
        if pin.is_low() {
            log::info!("Landing is detected.");
        }
        FreeRtos::delay_ms(1000);
    }
}

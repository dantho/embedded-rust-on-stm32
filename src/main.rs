#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::peripherals;
use embassy_nrf::uarte::{self, Config, Uarte};
use embassy_nrf::bind_interrupts;
use embassy_time::{Duration, Timer};
use panic_probe as _;

// 1. BIND INTERRUPTS
// The UART hardware generates interrupts when transmission finishes.
// We must link the specific peripheral (UARTE1) to the Embassy handler.
bind_interrupts!(struct Irqs {
    UARTE1 => uarte::InterruptHandler<peripherals::UARTE1>;
});

// 2. DEFINE THE DATA
// We create a large constant string to send over UART.
// In a real application, this could be a mutable buffer holding sensor data, logs, or any information you want to transmit.
// This string is stored in Flash memory, but DMA can read from Flash just as easily as RAM.
const LARGE_LOG_MESSAGE: &[u8] = b"This is a large string being sent over UART.
[INFO] System Boot Sequence Initiated...
[INFO] Initializing Sensors...
[INFO] Sensor 1: OK
[INFO] Sensor 2: OK
[INFO] Sensor 3: OK
[INFO] All sensors initialized successfully.
[INFO] Starting main application loop...
[INFO] Entering low-power mode...
[INFO] Waking up from low-power mode...
[INFO] Reading sensor data...
[INFO] Sensor 1: 23.5 deg C
[INFO] Sensor 2: 45.2% RH
[INFO] Sensor 3: 1013 hPa
[INFO] Sending data over UART...
";

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize the microcontroller peripherals
    let p = embassy_nrf::init(Default::default());

    // 3. CONFIGURE THE UART WITH DMA
    // We set up the UART peripheral with the desired baud rate and settings.
    let config = Config::default();

    // We initialize the UARTE peripheral with RX/TX pins and interrupt binding.
    let mut uart = Uarte::new(
        p.UARTE1,
        p.P1_12, // RX pin (D7 on XIAO nRF52840 Sense)
        p.P1_11, // TX pin (D6 on XIAO nRF52840 Sense)
        Irqs, // Interrupts for UARTE1
        config,
    );

    // We enter the application loop
    loop {
        // 4. SEND THE LARGE STRING
        // We use the `write` method to send the data over UART.
        // This method is async and will return immediately, allowing the CPU to do other work while the DMA handles the transmission.
        uart.write(LARGE_LOG_MESSAGE).await.unwrap();

        defmt::info!("Large log message sent over UART using DMA!");
        
        // 5. RESUME

        // When execution reaches this point, we know the DMA transmission is 100% complete.

        // 5. WAIT BEFORE SENDING AGAIN
        // We use a timer to create a delay between transmissions.
        // This simulates a real application where you might want to send data periodically.
        Timer::after(Duration::from_secs(5)).await;
    }
}

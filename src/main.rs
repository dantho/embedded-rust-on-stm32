#![no_std]
#![no_main]

// We import the panic handler for safety
// use panic_halt as _;

// We import the necessary Embassy components
use embassy_executor::Spawner;
use embassy_stm32::Peripherals;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, usart};
use embassy_stm32::timer::Timer;

// 1. BIND INTERRUPTS
// The UART hardware generates interrupts when transmission finishes.
// We must link the specific peripheral (USART2) to the Embassy handler.
bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<Peripherals::USART2>;
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
async fn main(spawner: Spawner) {
    // Initialize the microcontroller peripherals
    let p = embassy_stm32::init(Default::default());

    // 3. CONFIGURE THE UART WITH DMA
    // We set up the UART peripheral with the desired baud rate and settings.
    let mut config = Config::default();

    // We initialize the UART peripheral, and set up DMA for transmission.
    // Critical Step: we pass the DMA channels (DMA1_CH6 and DMA1_CH5).
    // Without these arguments, the driver would fall back to interrupt-driven transmission
    // By providing the DMA channels, we enable the Zero-Copy engine. 
    let mut uart = Uart::new(
        p.USART2, 
        p.PA3, // RX pin
        p.PA2, // TX pin
        Irqs,  // Interrupts for USART2
        p.DMA1_CH6, // DMA channel for TX
        p.DMA1_CH5, // DMA channel for RX (not used in this example, but required for full UART functionality)
        config,
    );

    // We enter the application loop
    loop {
        // 4. SEND THE LARGE STRING
        // We use the `write` method to send the data over UART.
        // This method is async and will return immediately, allowing the CPU to do other work while the DMA handles the transmission.
        uart.write(LARGE_LOG_MESSAGE).await.unwrap();

        // 5. RESUME

        // When execution reaches this point, we know the DMA transmission is 100% complete.

        // 5. WAIT BEFORE SENDING AGAIN
        // We use a timer to create a delay between transmissions.
        // This simulates a real application where you might want to send data periodically.
        Timer::after(core::time::Duration::from_secs(5)).await;
    }
}

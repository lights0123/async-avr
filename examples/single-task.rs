//! This example opens a serial connection to the host computer.  On most POSIX operating systems (like GNU/Linux or
//! OSX), you can interface with the program by running (assuming the device appears as ttyACM0)
//!
//! $ sudo screen /dev/ttyACM0 57600

#![no_std]
#![no_main]

use panic_halt as _;

use arduino_uno::prelude::*;

use async_avr::io::AsyncWriteExt;
use async_avr::{block_on, AsyncSerial};

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut serial = AsyncSerial::new(arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    ));

    block_on(async {
        loop {
            serial.write_all(b"Hello World!\n").await.unwrap();
        }
    });
    loop {}
}

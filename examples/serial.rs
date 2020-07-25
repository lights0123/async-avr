//! This example demonstrates how to set up a SPI interface and communicate
//! over it.  The physical hardware configuation consists of connecting a
//! jumper directly from pin `~11` to pin `~12`.
//!
//! This example opens a serial connection to the host computer.  On most POSIX operating systems (like GNU/Linux or
//! OSX), you can interface with the program by running (assuming the device appears as ttyACM0)
//!
//! $ sudo screen /dev/ttyACM0 57600#

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;

use core::cell::{Cell, RefCell};

use arduino_uno::spi::{Settings, Spi};

use async_avr::io::{AsyncReadExt, AsyncWriteExt};
use async_avr::{block_on, AsyncSerial, AsyncSpi, Yield};

#[no_mangle]
pub extern "C" fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    pins.d10.into_output(&mut pins.ddr); // CS must be set to output mode.

    // Create SPI interface.
    let spi = Spi::new(
        dp.SPI,
        pins.d13.into_output(&mut pins.ddr),
        pins.d11.into_output(&mut pins.ddr),
        pins.d12.into_pull_up_input(&mut pins.ddr),
        Settings::default(),
    );

    let mut spi = AsyncSpi::new(spi);

    let (rx, tx) = serial.split();
    let mut rx = AsyncSerial::new(rx);
    let tx = RefCell::new(AsyncSerial::new(tx));

    let serial_lock = Cell::new(false);
    let prio = Cell::new(false);

    let serial_loop = async {
        loop {
            let mut b = [0];
            rx.read_exact(&mut b).await.unwrap();
            loop {
                if !prio.get() && !serial_lock.get() {
                    serial_lock.set(true);
                    tx.borrow_mut().write_all(b"hello!\n").await.unwrap();
                    serial_lock.set(false);
                    break;
                }
                Yield::default().await;
            }
        }
    };

    let spi_loop = async {
        loop {
            spi.write_all(b"a").await.unwrap();
            let mut data = [0; 1];
            spi.read_exact(&mut data).await.unwrap();
            prio.set(true);
            loop {
                if !serial_lock.get() {
                    serial_lock.set(true);
                    let mut tx_ref = tx.borrow_mut();
                    tx_ref.write_all(b"wrote ").await.unwrap();
                    tx_ref.write_all(&data).await.unwrap();
                    tx_ref.write_all(b"!\n").await.unwrap();
                    serial_lock.set(false);
                    break;
                }
                Yield::default().await;
            }
            prio.set(false);
            Yield::default().await;
        }
    };

    block_on(async { futures_util::join!(serial_loop, spi_loop) });
    loop {}
}

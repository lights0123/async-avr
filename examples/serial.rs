#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;

use core::cell::{Cell, RefCell};
use core::ptr::{read_volatile, write_volatile};

use arduino_uno::prelude::*;
use arduino_uno::spi::{Settings, Spi};
use avr_hal_generic::nb;
use avr_hal_generic::port::mode::Output;

use async_avr::{AsyncSerial, AsyncSpi, block_on, Yield};
use async_avr::io::{AsyncReadExt, AsyncWriteExt};

// This example opens a serial connection to the host computer.  On most POSIX operating systems (like GNU/Linux or
// OSX), you can interface with the program by running (assuming the device appears as ttyACM0)
//
// $ sudo screen /dev/ttyACM0 57600

#[link_name = "__vector_18"]
pub unsafe extern "avr-interrupt" fn __vector_18() {
	let mut led: atmega328p_hal::port::portb::PB5<Output> = unsafe { core::mem::zeroed() };
	led.set_high();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
	let dp = arduino_uno::Peripherals::take().unwrap();

	let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

	dp.USART0.ucsr0b.write(|w| w.rxcie0().set_bit());

	let mut serial = arduino_uno::Serial::new(
		dp.USART0,
		pins.d0,
		pins.d1.into_output(&mut pins.ddr),
		57600,
	);

	avr_device::interrupt::enable();

	let mut led = pins.d13.into_output(&mut pins.ddr);

	// pins.d10.into_output(&mut pins.ddr); // SS must be set to output mode.
	//
	// // Create SPI interface.
	// let mut spi = Spi::new(
	//     dp.SPI,
	//     pins.d13.into_output(&mut pins.ddr),
	//     pins.d11.into_output(&mut pins.ddr),
	//     pins.d12.into_pull_up_input(&mut pins.ddr),
	//     Settings::default(),
	// );
	//
	// let mut spi = AsyncSpi::new(spi);

	let (rx, tx) = serial.split();
	let mut rx = AsyncSerial::new(rx);
	let mut tx = RefCell::new(AsyncSerial::new(tx));

	let mut serial_lock = Cell::new(false);
	let mut prio = Cell::new(false);

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
		// loop {
		//     // spi.write_all(b"az").await.unwrap();
		//     let mut data = [0; 2];
		//     spi.read_exact(&mut data).await.unwrap();
		//     prio.set(true);
		//     loop {
		//         if !serial_lock.get() {
		//             serial_lock.set(true);
		//             let mut tx_ref = tx.borrow_mut();
		//             tx_ref.write_all(b"wrote ").await.unwrap();
		//             tx_ref.write_all(&data).await.unwrap();
		//             tx_ref.write_all(b"!\n").await.unwrap();
		//             serial_lock.set(false);
		//             break;
		//         }
		//         Yield::default().await;
		//     }
		//     prio.set(false);
		//     Yield::default().await;
		// }
	};

	block_on(async { futures_util::join!(serial_loop, spi_loop) });
	loop {}

	// ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();
	//
	// loop {
	// 	// Read a byte from the serial connection
	// 	let b = nb::block!(serial.read()).unwrap();
	//
	// 	// Answer
	// 	ufmt::uwriteln!(&mut serial, "Got {}!\r", b).unwrap();
	// }
}

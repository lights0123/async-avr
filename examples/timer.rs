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

use panic_halt as _;
use core::cell::RefCell;
use avr_hal_generic::void::ResultVoidExt;

use async_avr::timer::TimeReader;

use async_avr::{block_on, timer};

struct ArduinoOneHwTimer {
    timer: arduino_hal::pac::TC1,
}

impl timer::TimeReader for ArduinoOneHwTimer {
    fn ticks(&self) -> u16 {
        return self.timer.tcnt1.read().bits();
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    let mut led = pins.d13.into_output();

    dp.TC1.tccr1b.write(|w| w.cs1().prescale_1024());
    let hw_timer = RefCell::new(ArduinoOneHwTimer { timer: dp.TC1 });

    let led_blink_1s = async {
        loop {
            led.toggle();
            timer::Timer::sleep_in_millis(1000u16, hw_timer.borrow()).await;
        }
    };

    let serial_write_3_5s = async {
        loop {
            ufmt::uwriteln!(&mut serial, "Another 3.5s have passed! {}\r", hw_timer.borrow().ticks()).void_unwrap();
            timer::Timer::sleep_in_millis(3500u16, hw_timer.borrow()).await;
        }
    };

    block_on(async { futures_util::join!(led_blink_1s, serial_write_3_5s) });
    loop {}
}

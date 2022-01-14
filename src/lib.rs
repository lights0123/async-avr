#![no_std]

use core::pin::Pin;
use core::task::{Context, Poll};

use avr_hal_generic::hal;
use avr_hal_generic::nb;

mod executor;
pub mod io;
mod spi;
pub use executor::block_on;
use futures_util::future::Future;
pub use spi::AsyncSpi;

pub mod timer;

pub struct AsyncSerial<T>(T);

impl<T> AsyncSerial<T> {
    pub fn new(serial: T) -> Self {
        serial.into()
    }
}

impl<T> From<T> for AsyncSerial<T> {
    fn from(serial: T) -> Self {
        AsyncSerial(serial)
    }
}

impl<T: hal::serial::Read<u8> + Unpin> io::AsyncRead for AsyncSerial<T> {
    type Error = T::Error;

    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, T::Error>> {
        if let Some(ptr) = buf.first_mut() {
            match self.0.read() {
                Ok(byte) => {
                    *ptr = byte;
                    Poll::Ready(Ok(1))
                }
                Err(nb::Error::WouldBlock) => Poll::Pending,
                Err(nb::Error::Other(err)) => Poll::Ready(Err(err)),
            }
        } else {
            Poll::Ready(Ok(0))
        }
    }
}

impl<T: hal::serial::Write<u8> + Unpin> io::AsyncWrite for AsyncSerial<T> {
    type Error = T::Error;

    fn poll_write(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, T::Error>> {
        if let Some(byte) = buf.first() {
            match self.0.write(*byte) {
                Ok(()) => Poll::Ready(Ok(1)),
                Err(nb::Error::WouldBlock) => Poll::Pending,
                Err(nb::Error::Other(err)) => Poll::Ready(Err(err)),
            }
        } else {
            Poll::Ready(Ok(0))
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), T::Error>> {
        match self.0.flush() {
            Ok(()) => Poll::Ready(Ok(())),
            Err(nb::Error::WouldBlock) => Poll::Pending,
            Err(nb::Error::Other(err)) => Poll::Ready(Err(err)),
        }
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), T::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub struct Yield(bool);

impl Default for Yield {
    fn default() -> Self {
        Yield(false)
    }
}

impl Future for Yield {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            Poll::Pending
        }
    }
}

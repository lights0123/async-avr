use crate::io;

use avr_hal_generic::hal;
use avr_hal_generic::nb;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct AsyncSpi<T>(T);

impl<T> AsyncSpi<T> {
    pub fn new(serial: T) -> Self {
        serial.into()
    }
}

impl<T> From<T> for AsyncSpi<T> {
    fn from(serial: T) -> Self {
        AsyncSpi(serial)
    }
}

impl<T: hal::spi::FullDuplex<u8> + Unpin> io::AsyncRead for AsyncSpi<T> {
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

impl<T: hal::spi::FullDuplex<u8> + Unpin> io::AsyncWrite for AsyncSpi<T> {
    type Error = T::Error;

    fn poll_write(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, T::Error>> {
        if let Some(byte) = buf.first() {
            match self.0.send(*byte) {
                Ok(()) => Poll::Ready(Ok(1)),
                Err(nb::Error::WouldBlock) => Poll::Pending,
                Err(nb::Error::Other(err)) => Poll::Ready(Err(err)),
            }
        } else {
            Poll::Ready(Ok(0))
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), T::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), T::Error>> {
        Poll::Ready(Ok(()))
    }
}

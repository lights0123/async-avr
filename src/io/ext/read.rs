use crate::io::AsyncRead;
use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_util::ready;

/// Future for the [`read`](super::AsyncReadExt::read) method.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Read<'a, R: ?Sized> {
    reader: &'a mut R,
    buf: &'a mut [u8],
}

impl<R: ?Sized + Unpin> Unpin for Read<'_, R> {}

impl<'a, R: AsyncRead + ?Sized + Unpin> Read<'a, R> {
    pub(super) fn new(reader: &'a mut R, buf: &'a mut [u8]) -> Self {
        Read { reader, buf }
    }
}

impl<R: AsyncRead + ?Sized + Unpin> Future for Read<'_, R> {
    type Output = Result<usize, R::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        Pin::new(&mut this.reader).poll_read(cx, this.buf)
    }
}

/// Future for the [`read_exact`](super::AsyncReadExt::read_exact) method.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct ReadExact<'a, R: ?Sized> {
    reader: &'a mut R,
    buf: &'a mut [u8],
}

impl<R: ?Sized + Unpin> Unpin for ReadExact<'_, R> {}

impl<'a, R: AsyncRead + ?Sized + Unpin> ReadExact<'a, R> {
    pub(super) fn new(reader: &'a mut R, buf: &'a mut [u8]) -> Self {
        ReadExact { reader, buf }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ReadExactError<E> {
    UnexpectedEof,
    Other(E),
}

impl<E> From<E> for ReadExactError<E> {
    fn from(err: E) -> Self {
        ReadExactError::Other(err)
    }
}

impl<R: AsyncRead + ?Sized + Unpin> Future for ReadExact<'_, R> {
    type Output = Result<(), ReadExactError<R::Error>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        while !this.buf.is_empty() {
            let n = ready!(Pin::new(&mut this.reader).poll_read(cx, this.buf))?;
            {
                let (_, rest) = mem::replace(&mut this.buf, &mut []).split_at_mut(n);
                this.buf = rest;
            }
            if n == 0 {
                return Poll::Ready(Err(ReadExactError::UnexpectedEof));
            }
        }
        Poll::Ready(Ok(()))
    }
}

use crate::io::AsyncWrite;
use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_util::ready;

/// Future for the [`write`](super::AsyncWriteExt::write) method.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Write<'a, W: ?Sized> {
    writer: &'a mut W,
    buf: &'a [u8],
}

impl<W: ?Sized + Unpin> Unpin for Write<'_, W> {}

impl<'a, W: AsyncWrite + ?Sized + Unpin> Write<'a, W> {
    pub(super) fn new(writer: &'a mut W, buf: &'a [u8]) -> Self {
        Self { writer, buf }
    }
}

impl<W: AsyncWrite + ?Sized + Unpin> Future for Write<'_, W> {
    type Output = Result<usize, W::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        Pin::new(&mut this.writer).poll_write(cx, this.buf)
    }
}
/// Future for the [`write_all`](super::AsyncWriteExt::write_all) method.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WriteAll<'a, W: ?Sized> {
    writer: &'a mut W,
    buf: &'a [u8],
}

impl<W: ?Sized + Unpin> Unpin for WriteAll<'_, W> {}

impl<'a, W: AsyncWrite + ?Sized + Unpin> WriteAll<'a, W> {
    pub(super) fn new(writer: &'a mut W, buf: &'a [u8]) -> Self {
        WriteAll { writer, buf }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum WriteAllError<E> {
    WriteZero,
    Other(E),
}

impl<E> From<E> for WriteAllError<E> {
    fn from(err: E) -> Self {
        WriteAllError::Other(err)
    }
}

impl<W: AsyncWrite + ?Sized + Unpin> Future for WriteAll<'_, W> {
    type Output = Result<(), WriteAllError<W::Error>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        while !this.buf.is_empty() {
            let n = ready!(Pin::new(&mut this.writer).poll_write(cx, this.buf))?;
            {
                let (_, rest) = mem::replace(&mut this.buf, &[]).split_at(n);
                this.buf = rest;
            }
            if n == 0 {
                return Poll::Ready(Err(WriteAllError::WriteZero));
            }
        }

        Poll::Ready(Ok(()))
    }
}
/// Future for the [`flush`](super::AsyncWriteExt::flush) method.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Flush<'a, W: ?Sized> {
    writer: &'a mut W,
}

impl<W: ?Sized + Unpin> Unpin for Flush<'_, W> {}

impl<'a, W: AsyncWrite + ?Sized + Unpin> Flush<'a, W> {
    pub(super) fn new(writer: &'a mut W) -> Self {
        Flush { writer }
    }
}

impl<W> Future for Flush<'_, W>
where
    W: AsyncWrite + ?Sized + Unpin,
{
    type Output = Result<(), W::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut *self.writer).poll_flush(cx)
    }
}

/// Future for the [`close`](super::AsyncWriteExt::close) method.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Close<'a, W: ?Sized> {
    writer: &'a mut W,
}

impl<W: ?Sized + Unpin> Unpin for Close<'_, W> {}

impl<'a, W: AsyncWrite + ?Sized + Unpin> Close<'a, W> {
    pub(super) fn new(writer: &'a mut W) -> Self {
        Close { writer }
    }
}

impl<W: AsyncWrite + ?Sized + Unpin> Future for Close<'_, W> {
    type Output = Result<(), W::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut *self.writer).poll_close(cx)
    }
}

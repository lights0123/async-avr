use core::pin::Pin;
use core::task::{Context, Poll};
pub use ext::*;
mod ext;
/// Read bytes asynchronously.
///
/// This trait is analogous to the `std::io::Read` trait, but integrates
/// with the asynchronous task system. In particular, the `poll_read`
/// method, unlike `Read::read`, will automatically queue the current task
/// for wakeup and return if data is not yet available, rather than blocking
/// the calling thread.
pub trait AsyncRead {
    type Error;
    /// Determines if this `AsyncRead`er can work with buffers of
    /// uninitialized memory.
    ///
    /// The default implementation returns an initializer which will zero
    /// buffers.
    ///
    /// This method is only available when the `read-initializer` feature of this
    /// library is activated.
    ///
    /// # Safety
    ///
    /// This method is `unsafe` because an `AsyncRead`er could otherwise
    /// return a non-zeroing `Initializer` from another `AsyncRead` type
    /// without an `unsafe` block.
    #[cfg(feature = "read-initializer")]
    #[inline]
    unsafe fn initializer(&self) -> Initializer {
        Initializer::zeroing()
    }

    /// Attempt to read from the `AsyncRead` into `buf`.
    ///
    /// On success, returns `Poll::Ready(Ok(num_bytes_read))`.
    ///
    /// If no data is available for reading, the method returns
    /// `Poll::Pending` and arranges for the current task (via
    /// `cx.waker().wake_by_ref()`) to receive a notification when the object becomes
    /// readable or is closed.
    ///
    /// # Implementation
    ///
    /// This function may not return errors of kind `WouldBlock` or
    /// `Interrupted`.  Implementations must convert `WouldBlock` into
    /// `Poll::Pending` and either internally retry or convert
    /// `Interrupted` into another error kind.
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Self::Error>>;
}

impl<T: ?Sized + AsyncRead + Unpin> AsyncRead for &mut T {
    type Error = T::Error;

    #[cfg(feature = "read-initializer")]
    unsafe fn initializer(&self) -> Initializer {
        (**self).initializer()
    }

    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, T::Error>> {
        Pin::new(&mut **self).poll_read(cx, buf)
    }
}

/// Write bytes asynchronously.
///
/// This trait is analogous to the `std::io::Write` trait, but integrates
/// with the asynchronous task system. In particular, the `poll_write`
/// method, unlike `Write::write`, will automatically queue the current task
/// for wakeup and return if the writer cannot take more data, rather than blocking
/// the calling thread.
pub trait AsyncWrite {
    type Error;
    /// Attempt to write bytes from `buf` into the object.
    ///
    /// On success, returns `Poll::Ready(Ok(num_bytes_written))`.
    ///
    /// If the object is not ready for writing, the method returns
    /// `Poll::Pending` and arranges for the current task (via
    /// `cx.waker().wake_by_ref()`) to receive a notification when the object becomes
    /// writable or is closed.
    ///
    /// # Implementation
    ///
    /// This function may not return errors of kind `WouldBlock` or
    /// `Interrupted`.  Implementations must convert `WouldBlock` into
    /// `Poll::Pending` and either internally retry or convert
    /// `Interrupted` into another error kind.
    ///
    /// `poll_write` must try to make progress by flushing the underlying object if
    /// that is the only way the underlying object can become writable again.
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::Error>>;

    /// Attempt to flush the object, ensuring that any buffered data reach
    /// their destination.
    ///
    /// On success, returns `Poll::Ready(Ok(()))`.
    ///
    /// If flushing cannot immediately complete, this method returns
    /// `Poll::Pending` and arranges for the current task (via
    /// `cx.waker().wake_by_ref()`) to receive a notification when the object can make
    /// progress towards flushing.
    ///
    /// # Implementation
    ///
    /// This function may not return errors of kind `WouldBlock` or
    /// `Interrupted`.  Implementations must convert `WouldBlock` into
    /// `Poll::Pending` and either internally retry or convert
    /// `Interrupted` into another error kind.
    ///
    /// It only makes sense to do anything here if you actually buffer data.
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;

    /// Attempt to close the object.
    ///
    /// On success, returns `Poll::Ready(Ok(()))`.
    ///
    /// If closing cannot immediately complete, this function returns
    /// `Poll::Pending` and arranges for the current task (via
    /// `cx.waker().wake_by_ref()`) to receive a notification when the object can make
    /// progress towards closing.
    ///
    /// # Implementation
    ///
    /// This function may not return errors of kind `WouldBlock` or
    /// `Interrupted`.  Implementations must convert `WouldBlock` into
    /// `Poll::Pending` and either internally retry or convert
    /// `Interrupted` into another error kind.
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
}

impl<T: ?Sized + AsyncWrite + Unpin> AsyncWrite for &mut T {
    type Error = T::Error;

    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, T::Error>> {
        Pin::new(&mut **self).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), T::Error>> {
        Pin::new(&mut **self).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), T::Error>> {
        Pin::new(&mut **self).poll_close(cx)
    }
}

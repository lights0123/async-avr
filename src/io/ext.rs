use super::{AsyncRead, AsyncWrite};
use read::*;
use write::*;

mod read;
mod write;
/// An extension trait which adds utility methods to `AsyncRead` types.
pub trait AsyncReadExt: AsyncRead {
    // /// Creates an adaptor which will chain this stream with another.
    // ///
    // /// The returned `AsyncRead` instance will first read all bytes from this object
    // /// until EOF is encountered. Afterwards the output is equivalent to the
    // /// output of `next`.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// # futures::executor::block_on(async {
    // /// use futures::io::{AsyncReadExt, Cursor};
    // ///
    // /// let reader1 = Cursor::new([1, 2, 3, 4]);
    // /// let reader2 = Cursor::new([5, 6, 7, 8]);
    // ///
    // /// let mut reader = reader1.chain(reader2);
    // /// let mut buffer = Vec::new();
    // ///
    // /// // read the value into a Vec.
    // /// reader.read_to_end(&mut buffer).await?;
    // /// assert_eq!(buffer, [1, 2, 3, 4, 5, 6, 7, 8]);
    // /// # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();
    // /// ```
    // fn chain<R>(self, next: R) -> Chain<Self, R>
    // 	where
    // 		Self: Sized,
    // 		R: AsyncRead,
    // {
    // 	Chain::new(self, next)
    // }

    /// Tries to read some bytes directly into the given `buf` in asynchronous
    /// manner, returning a future type.
    ///
    /// The returned future will resolve to the number of bytes read once the read
    /// operation is completed.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use futures::io::{AsyncReadExt, Cursor};
    ///
    /// let mut reader = Cursor::new([1, 2, 3, 4]);
    /// let mut output = [0u8; 5];
    ///
    /// let bytes = reader.read(&mut output[..]).await?;
    ///
    /// // This is only guaranteed to be 4 because `&[u8]` is a synchronous
    /// // reader. In a real system you could get anywhere from 1 to
    /// // `output.len()` bytes in a single read.
    /// assert_eq!(bytes, 4);
    /// assert_eq!(output, [1, 2, 3, 4, 0]);
    /// # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();
    /// ```
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> Read<'a, Self>
    where
        Self: Unpin,
    {
        Read::new(self, buf)
    }

    // /// Creates a future which will read from the `AsyncRead` into `bufs` using vectored
    // /// IO operations.
    // ///
    // /// The returned future will resolve to the number of bytes read once the read
    // /// operation is completed.
    // fn read_vectored<'a>(&'a mut self, bufs: &'a mut [IoSliceMut<'a>]) -> ReadVectored<'a, Self>
    // 	where Self: Unpin,
    // {
    // 	ReadVectored::new(self, bufs)
    // }

    /// Creates a future which will read exactly enough bytes to fill `buf`,
    /// returning an error if end of file (EOF) is hit sooner.
    ///
    /// The returned future will resolve once the read operation is completed.
    ///
    /// In the case of an error the buffer and the object will be discarded, with
    /// the error yielded.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use futures::io::{AsyncReadExt, Cursor};
    ///
    /// let mut reader = Cursor::new([1, 2, 3, 4]);
    /// let mut output = [0u8; 4];
    ///
    /// reader.read_exact(&mut output).await?;
    ///
    /// assert_eq!(output, [1, 2, 3, 4]);
    /// # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();
    /// ```
    ///
    /// ## EOF is hit before `buf` is filled
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use futures::io::{self, AsyncReadExt, Cursor};
    ///
    /// let mut reader = Cursor::new([1, 2, 3, 4]);
    /// let mut output = [0u8; 5];
    ///
    /// let result = reader.read_exact(&mut output).await;
    ///
    /// assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    /// # });
    /// ```
    fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadExact<'a, Self>
    where
        Self: Unpin,
    {
        ReadExact::new(self, buf)
    }

    // /// Creates a future which will read all the bytes from this `AsyncRead`.
    // ///
    // /// On success the total number of bytes read is returned.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// # futures::executor::block_on(async {
    // /// use futures::io::{AsyncReadExt, Cursor};
    // ///
    // /// let mut reader = Cursor::new([1, 2, 3, 4]);
    // /// let mut output = Vec::with_capacity(4);
    // ///
    // /// let bytes = reader.read_to_end(&mut output).await?;
    // ///
    // /// assert_eq!(bytes, 4);
    // /// assert_eq!(output, vec![1, 2, 3, 4]);
    // /// # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();
    // /// ```
    // fn read_to_end<'a>(
    // 	&'a mut self,
    // 	buf: &'a mut Vec<u8>,
    // ) -> ReadToEnd<'a, Self>
    // 	where Self: Unpin,
    // {
    // 	ReadToEnd::new(self, buf)
    // }
    //
    // /// Creates a future which will read all the bytes from this `AsyncRead`.
    // ///
    // /// On success the total number of bytes read is returned.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// # futures::executor::block_on(async {
    // /// use futures::io::{AsyncReadExt, Cursor};
    // ///
    // /// let mut reader = Cursor::new(&b"1234"[..]);
    // /// let mut buffer = String::with_capacity(4);
    // ///
    // /// let bytes = reader.read_to_string(&mut buffer).await?;
    // ///
    // /// assert_eq!(bytes, 4);
    // /// assert_eq!(buffer, String::from("1234"));
    // /// # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();
    // /// ```
    // fn read_to_string<'a>(
    // 	&'a mut self,
    // 	buf: &'a mut String,
    // ) -> ReadToString<'a, Self>
    // 	where Self: Unpin,
    // {
    // 	ReadToString::new(self, buf)
    // }

    // /// Helper method for splitting this read/write object into two halves.
    // ///
    // /// The two halves returned implement the `AsyncRead` and `AsyncWrite`
    // /// traits, respectively.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// # futures::executor::block_on(async {
    // /// use futures::io::{self, AsyncReadExt, Cursor};
    // ///
    // /// // Note that for `Cursor` the read and write halves share a single
    // /// // seek position. This may or may not be true for other types that
    // /// // implement both `AsyncRead` and `AsyncWrite`.
    // ///
    // /// let reader = Cursor::new([1, 2, 3, 4]);
    // /// let mut buffer = Cursor::new(vec![0, 0, 0, 0, 5, 6, 7, 8]);
    // /// let mut writer = Cursor::new(vec![0u8; 5]);
    // ///
    // /// {
    // ///     let (buffer_reader, mut buffer_writer) = (&mut buffer).split();
    // ///     io::copy(reader, &mut buffer_writer).await?;
    // ///     io::copy(buffer_reader, &mut writer).await?;
    // /// }
    // ///
    // /// assert_eq!(buffer.into_inner(), [1, 2, 3, 4, 5, 6, 7, 8]);
    // /// assert_eq!(writer.into_inner(), [5, 6, 7, 8, 0]);
    // /// # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();
    // /// ```
    // fn split(self) -> (ReadHalf<Self>, WriteHalf<Self>)
    // 	where Self: AsyncWrite + Sized,
    // {
    // 	split::split(self)
    // }

    // /// Creates an AsyncRead adapter which will read at most `limit` bytes
    // /// from the underlying reader.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// # futures::executor::block_on(async {
    // /// use futures::io::{AsyncReadExt, Cursor};
    // ///
    // /// let reader = Cursor::new(&b"12345678"[..]);
    // /// let mut buffer = [0; 5];
    // ///
    // /// let mut take = reader.take(4);
    // /// let n = take.read(&mut buffer).await?;
    // ///
    // /// assert_eq!(n, 4);
    // /// assert_eq!(&buffer, b"1234\0");
    // /// # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();
    // /// ```
    // fn take(self, limit: u64) -> Take<Self>
    // 	where Self: Sized
    // {
    // 	Take::new(self, limit)
    // }
}

impl<R: AsyncRead + ?Sized> AsyncReadExt for R {}

/// An extension trait which adds utility methods to `AsyncWrite` types.
pub trait AsyncWriteExt: AsyncWrite {
    /// Creates a future which will entirely flush this `AsyncWrite`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use futures::io::{AllowStdIo, AsyncWriteExt};
    /// use std::io::{BufWriter, Cursor};
    ///
    /// let mut output = vec![0u8; 5];
    ///
    /// {
    ///     let writer = Cursor::new(&mut output);
    ///     let mut buffered = AllowStdIo::new(BufWriter::new(writer));
    ///     buffered.write_all(&[1, 2]).await?;
    ///     buffered.write_all(&[3, 4]).await?;
    ///     buffered.flush().await?;
    /// }
    ///
    /// assert_eq!(output, [1, 2, 3, 4, 0]);
    /// # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();
    /// ```
    fn flush(&mut self) -> Flush<'_, Self>
    where
        Self: Unpin,
    {
        Flush::new(self)
    }

    /// Creates a future which will entirely close this `AsyncWrite`.
    fn close(&mut self) -> Close<'_, Self>
    where
        Self: Unpin,
    {
        Close::new(self)
    }

    /// Creates a future which will write bytes from `buf` into the object.
    ///
    /// The returned future will resolve to the number of bytes written once the write
    /// operation is completed.
    fn write<'a>(&'a mut self, buf: &'a [u8]) -> Write<'a, Self>
    where
        Self: Unpin,
    {
        Write::new(self, buf)
    }

    // /// Creates a future which will write bytes from `bufs` into the object using vectored
    // /// IO operations.
    // ///
    // /// The returned future will resolve to the number of bytes written once the write
    // /// operation is completed.
    // fn write_vectored<'a>(&'a mut self, bufs: &'a [IoSlice<'a>]) -> WriteVectored<'a, Self>
    // 	where Self: Unpin,
    // {
    // 	WriteVectored::new(self, bufs)
    // }

    /// Write data into this object.
    ///
    /// Creates a future that will write the entire contents of the buffer `buf` into
    /// this `AsyncWrite`.
    ///
    /// The returned future will not complete until all the data has been written.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use futures::io::{AsyncWriteExt, Cursor};
    ///
    /// let mut writer = Cursor::new(vec![0u8; 5]);
    ///
    /// writer.write_all(&[1, 2, 3, 4]).await?;
    ///
    /// assert_eq!(writer.into_inner(), [1, 2, 3, 4, 0]);
    /// # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();
    /// ```
    fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> WriteAll<'a, Self>
    where
        Self: Unpin,
    {
        WriteAll::new(self, buf)
    }

    // /// Attempts to write multiple buffers into this writer.
    // ///
    // /// Creates a future that will write the entire contents of `bufs` into this
    // /// `AsyncWrite` using [vectored writes].
    // ///
    // /// The returned future will not complete until all the data has been
    // /// written.
    // ///
    // /// [vectored writes]: std::io::Write::write_vectored
    // ///
    // /// # Notes
    // ///
    // /// Unlike `io::Write::write_vectored`, this takes a *mutable* reference to
    // /// a slice of `IoSlice`s, not an immutable one. That's because we need to
    // /// modify the slice to keep track of the bytes already written.
    // ///
    // /// Once this futures returns, the contents of `bufs` are unspecified, as
    // /// this depends on how many calls to `write_vectored` were necessary. It is
    // /// best to understand this function as taking ownership of `bufs` and to
    // /// not use `bufs` afterwards. The underlying buffers, to which the
    // /// `IoSlice`s point (but not the `IoSlice`s themselves), are unchanged and
    // /// can be reused.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// # futures::executor::block_on(async {
    // /// use futures::io::AsyncWriteExt;
    // /// use std::io::{Cursor, IoSlice};
    // ///
    // /// let mut writer = Cursor::new([0u8; 7]);
    // /// let bufs = &mut [
    // ///     IoSlice::new(&[1]),
    // ///     IoSlice::new(&[2, 3]),
    // ///     IoSlice::new(&[4, 5, 6]),
    // /// ];
    // ///
    // /// writer.write_all_vectored(bufs).await?;
    // /// // Note: the contents of `bufs` is now undefined, see the Notes section.
    // ///
    // /// assert_eq!(writer.into_inner(), [1, 2, 3, 4, 5, 6, 0]);
    // /// # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();
    // /// ```
    // #[cfg(feature = "write_all_vectored")]
    // fn write_all_vectored<'a>(
    // 	&'a mut self,
    // 	bufs: &'a mut [IoSlice<'a>],
    // ) -> WriteAllVectored<'a, Self>
    // 	where
    // 		Self: Unpin,
    // {
    // 	WriteAllVectored::new(self, bufs)
    // }

    /// Allow using an [`AsyncWrite`] as a [`Sink`](futures_sink::Sink)`<Item: AsRef<[u8]>>`.
    ///
    /// This adapter produces a sink that will write each value passed to it
    /// into the underlying writer.
    ///
    /// Note that this function consumes the given writer, returning a wrapped
    /// version.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use futures::io::AsyncWriteExt;
    /// use futures::stream::{self, StreamExt};
    ///
    /// let stream = stream::iter(vec![Ok([1, 2, 3]), Ok([4, 5, 6])]);
    ///
    /// let mut writer = vec![];
    ///
    /// stream.forward((&mut writer).into_sink()).await?;
    ///
    /// assert_eq!(writer, vec![1, 2, 3, 4, 5, 6]);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # })?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[cfg(feature = "sink")]
    fn into_sink<Item: AsRef<[u8]>>(self) -> IntoSink<Self, Item>
    where
        Self: Sized,
    {
        IntoSink::new(self)
    }
}

impl<W: AsyncWrite + ?Sized> AsyncWriteExt for W {}

use std::io::{BufRead, Read, Write};

use crate::Muncher;

impl<T: Read> Read for Muncher<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<T: BufRead> BufRead for Muncher<T> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amount: usize) {
        self.inner.consume(amount);
    }
}

impl<T: Write> Write for Muncher<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(feature = "futures")]
use futures::io::{AsyncRead, AsyncWrite};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead, AsyncWrite};

#[cfg(feature = "tokio")]
type PollReadOut = std::task::Poll<std::io::Result<()>>;
#[cfg(feature = "futures")]
type PollReadOut = std::task::Poll<std::io::Result<usize>>;

#[cfg(any(feature = "tokio", feature = "futures"))]
impl<T: AsyncRead + Unpin> AsyncRead for Muncher<T> {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        #[cfg(feature = "tokio")] buf: &mut tokio::io::ReadBuf<'_>,
        #[cfg(feature = "futures")] buf: &mut [u8],
    ) -> PollReadOut {
        let pinned = std::pin::pin!(&mut self.inner);
        pinned.poll_read(cx, buf)
    }
}

#[cfg(any(feature = "tokio", feature = "futures"))]
impl<T: AsyncWrite + Unpin> AsyncWrite for Muncher<T> {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        let pinned = std::pin::pin!(&mut self.inner);
        pinned.poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let pinned = std::pin::pin!(&mut self.inner);
        pinned.poll_flush(cx)
    }

    #[cfg(feature = "tokio")]
    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let pinned = std::pin::pin!(&mut self.inner);
        pinned.poll_shutdown(cx)
    }

    #[cfg(feature = "futures")]
    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let pinned = std::pin::pin!(&mut self.inner);
        pinned.poll_close(cx)
    }
}

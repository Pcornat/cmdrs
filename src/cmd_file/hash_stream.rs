use std::{borrow::BorrowMut, pin::Pin, task::Poll};

use crypto::digest::Digest;
use tokio::io::{AsyncRead, AsyncWrite};

pub(super) struct HashingStream<D, I> {
    pub(super) digest: D,
    pub(super) inner: I,
}

macro_rules! impl_all {
    ($x:ident ; $e:ident) => {$x};
    ($($e:ident)+) => {

        impl<$($e: Digest + Unpin,)+ Inner: AsyncRead + Unpin> AsyncRead for HashingStream<($($e,)+), Inner> {
            #[allow(non_snake_case)]
            fn poll_read(
                mut self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &mut tokio::io::ReadBuf<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                let poll = Pin::new(self.inner.borrow_mut()).poll_read(cx, buf);

                if let Poll::Ready(Ok(_)) = poll {
                    let ($($e,)+) = &mut self.digest;
                    $($e.input(buf.filled());)+
                }

                poll
            }
        }

        impl<$($e: Digest + Unpin,)+ Inner: AsyncWrite + Unpin> AsyncWrite for HashingStream<($($e,)+), Inner> {
            #[allow(non_snake_case)]
            fn poll_write(
                mut self: Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &[u8],
            ) -> Poll<Result<usize, std::io::Error>> {
                let poll = Pin::new(self.inner.borrow_mut()).poll_write(cx, buf);

                if let Poll::Ready(Ok(written)) = poll {
                    let ($($e,)+) = &mut self.digest;
                    $($e.input(&buf[0..written]);)+
                }

                poll
            }

            fn poll_flush(
                mut self: Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> Poll<Result<(), std::io::Error>> {
                Pin::new(self.inner.borrow_mut()).poll_flush(cx)
            }

            fn poll_shutdown(
                mut self: Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> Poll<Result<(), std::io::Error>> {
                Pin::new(self.inner.borrow_mut()).poll_shutdown(cx)
            }
        }

        impl<$($e: Digest + Unpin,)+ Inner> HashingStream<($($e,)+), Inner> {
            #[allow(non_snake_case, dead_code)]
            pub(super) fn fingerprints(&mut self) -> ($(impl_all!(String; $e),)+) {
                let ($($e,)+) = &mut self.digest;
                ($($e.result_str(),)+)
            }
        }


    };
}

impl_all!(A);
impl_all!(A B);
impl_all!(A B C);
impl_all!(A B C D);
impl_all!(A B C D E);
impl_all!(A B C D E F);
impl_all!(A B C D E F G);
impl_all!(A B C D E F G H);
impl_all!(A B C D E F G H I);
impl_all!(A B C D E F G H I J);
impl_all!(A B C D E F G H I J K);
impl_all!(A B C D E F G H I J K L);

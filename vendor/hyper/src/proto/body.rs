use bytes::Bytes;
use futures::{Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};
use futures::sync::{mpsc, oneshot};
#[cfg(feature = "tokio-proto")]
use tokio_proto;
use std::borrow::Cow;

use super::Chunk;

#[cfg(feature = "tokio-proto")]
pub type TokioBody = tokio_proto::streaming::Body<Chunk, ::Error>;
pub type BodySender = mpsc::Sender<Result<Chunk, ::Error>>;

/// A `Stream` for `Chunk`s used in requests and responses.
#[must_use = "streams do nothing unless polled"]
#[derive(Debug)]
pub struct Body(Inner);

#[derive(Debug)]
enum Inner {
    #[cfg(feature = "tokio-proto")]
    Tokio(TokioBody),
    Chan {
        close_tx: oneshot::Sender<bool>,
        rx: mpsc::Receiver<Result<Chunk, ::Error>>,
    },
    Once(Option<Chunk>),
    Empty,
}

//pub(crate)
#[derive(Debug)]
pub struct ChunkSender {
    close_rx: oneshot::Receiver<bool>,
    close_rx_check: bool,
    tx: BodySender,
}

impl Body {
    /// Return an empty body stream
    #[inline]
    pub fn empty() -> Body {
        Body(Inner::Empty)
    }

    /// Return a body stream with an associated sender half
    #[inline]
    pub fn pair() -> (mpsc::Sender<Result<Chunk, ::Error>>, Body) {
        let (tx, rx) = channel();
        (tx.tx, rx)
    }
}

impl Default for Body {
    #[inline]
    fn default() -> Body {
        Body::empty()
    }
}

impl Stream for Body {
    type Item = Chunk;
    type Error = ::Error;

    #[inline]
    fn poll(&mut self) -> Poll<Option<Chunk>, ::Error> {
        match self.0 {
            #[cfg(feature = "tokio-proto")]
            Inner::Tokio(ref mut rx) => rx.poll(),
            Inner::Chan { ref mut rx, .. } => match rx.poll().expect("mpsc cannot error") {
                Async::Ready(Some(Ok(chunk))) => Ok(Async::Ready(Some(chunk))),
                Async::Ready(Some(Err(err))) => Err(err),
                Async::Ready(None) => Ok(Async::Ready(None)),
                Async::NotReady => Ok(Async::NotReady),
            },
            Inner::Once(ref mut val) => Ok(Async::Ready(val.take())),
            Inner::Empty => Ok(Async::Ready(None)),
        }
    }
}

//pub(crate)
pub fn channel() -> (ChunkSender, Body) {
    let (tx, rx) = mpsc::channel(0);
    let (close_tx, close_rx) = oneshot::channel();

    let tx = ChunkSender {
        close_rx: close_rx,
        close_rx_check: true,
        tx: tx,
    };
    let rx = Body(Inner::Chan {
        close_tx: close_tx,
        rx: rx,
    });

    (tx, rx)
}

impl ChunkSender {
    pub fn poll_ready(&mut self) -> Poll<(), ()> {
        if self.close_rx_check {
            match self.close_rx.poll() {
                Ok(Async::Ready(true)) | Err(_) => return Err(()),
                Ok(Async::Ready(false)) => {
                    // needed to allow converting into a plain mpsc::Receiver
                    // if it has been, the tx will send false to disable this check
                    self.close_rx_check = false;
                }
                Ok(Async::NotReady) => (),
            }
        }

        self.tx.poll_ready().map_err(|_| ())
    }

    pub fn start_send(&mut self, msg: Result<Chunk, ::Error>) -> StartSend<(), ()> {
        match self.tx.start_send(msg) {
            Ok(AsyncSink::Ready) => Ok(AsyncSink::Ready),
            Ok(AsyncSink::NotReady(_)) => Ok(AsyncSink::NotReady(())),
            Err(_) => Err(()),
        }
    }
}

feat_server_proto! {
    impl From<Body> for tokio_proto::streaming::Body<Chunk, ::Error> {
        fn from(b: Body) -> tokio_proto::streaming::Body<Chunk, ::Error> {
            match b.0 {
                Inner::Tokio(b) => b,
                Inner::Chan { close_tx, rx } => {
                    // disable knowing if the Rx gets dropped, since we cannot
                    // pass this tx along.
                    let _ = close_tx.send(false);
                    rx.into()
                },
                Inner::Once(Some(chunk)) => TokioBody::from(chunk),
                Inner::Once(None) |
                Inner::Empty => TokioBody::empty(),
            }
        }
    }

    impl From<tokio_proto::streaming::Body<Chunk, ::Error>> for Body {
        fn from(tokio_body: tokio_proto::streaming::Body<Chunk, ::Error>) -> Body {
            Body(Inner::Tokio(tokio_body))
        }
    }
}

impl From<mpsc::Receiver<Result<Chunk, ::Error>>> for Body {
    #[inline]
    fn from(src: mpsc::Receiver<Result<Chunk, ::Error>>) -> Body {
        let (tx, _) = oneshot::channel();
        Body(Inner::Chan {
            close_tx: tx,
            rx: src,
        })
    }
}

impl From<Chunk> for Body {
    #[inline]
    fn from (chunk: Chunk) -> Body {
        Body(Inner::Once(Some(chunk)))
    }
}

impl From<Bytes> for Body {
    #[inline]
    fn from (bytes: Bytes) -> Body {
        Body::from(Chunk::from(bytes))
    }
}

impl From<Vec<u8>> for Body {
    #[inline]
    fn from (vec: Vec<u8>) -> Body {
        Body::from(Chunk::from(vec))
    }
}

impl From<&'static [u8]> for Body {
    #[inline]
    fn from (slice: &'static [u8]) -> Body {
        Body::from(Chunk::from(slice))
    }
}

impl From<Cow<'static, [u8]>> for Body {
    #[inline]
    fn from (cow: Cow<'static, [u8]>) -> Body {
        match cow {
            Cow::Borrowed(b) => Body::from(b),
            Cow::Owned(o) => Body::from(o)
        }
    }
}

impl From<String> for Body {
    #[inline]
    fn from (s: String) -> Body {
        Body::from(Chunk::from(s.into_bytes()))
    }
}

impl From<&'static str> for Body {
    #[inline]
    fn from(slice: &'static str) -> Body {
        Body::from(Chunk::from(slice.as_bytes()))
    }
}

impl From<Cow<'static, str>> for Body {
    #[inline]
    fn from(cow: Cow<'static, str>) -> Body {
        match cow {
            Cow::Borrowed(b) => Body::from(b),
            Cow::Owned(o) => Body::from(o)
        }
    }
}

impl From<Option<Body>> for Body {
    #[inline]
    fn from (body: Option<Body>) -> Body {
        body.unwrap_or_default()
    }
}

fn _assert_send_sync() {
    fn _assert_send<T: Send>() {}
    fn _assert_sync<T: Sync>() {}

    _assert_send::<Body>();
    _assert_send::<Chunk>();
    _assert_sync::<Chunk>();
}

#[test]
fn test_body_stream_concat() {
    use futures::{Sink, Stream, Future};
    let (tx, body) = Body::pair();

    ::std::thread::spawn(move || {
        let tx = tx.send(Ok("hello ".into())).wait().unwrap();
        tx.send(Ok("world".into())).wait().unwrap();
    });

    let total = body.concat2().wait().unwrap();
    assert_eq!(total.as_ref(), b"hello world");

}

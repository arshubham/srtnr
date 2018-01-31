use std::fs::File;
use std::fmt;
use std::io::{self, Cursor, Read};

use bytes::Bytes;
use hyper::{self, Chunk};

use {async_impl, wait};

/// The body of a `Request`.
///
/// In most cases, this is not needed directly, as the
/// [`RequestBuilder.body`][builder] method uses `Into<Body>`, which allows
/// passing many things (like a string or vector of bytes).
///
/// [builder]: ./struct.RequestBuilder.html#method.body
#[derive(Debug)]
pub struct Body {
    kind: Kind,
}

impl Body {
    /// Instantiate a `Body` from a reader.
    ///
    /// # Note
    ///
    /// While allowing for many types to be used, these bodies do not have
    /// a way to reset to the beginning and be reused. This means that when
    /// encountering a 307 or 308 status code, instead of repeating the
    /// request at the new location, the `Response` will be returned with
    /// the redirect status code set.
    ///
    /// ```rust
    /// # use std::fs::File;
    /// # use reqwest::Body;
    /// # fn run() -> Result<(), Box<std::error::Error>> {
    /// let file = File::open("national_secrets.txt")?;
    /// let body = Body::new(file);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// If you have a set of bytes, like `String` or `Vec<u8>`, using the
    /// `From` implementations for `Body` will store the data in a manner
    /// it can be reused.
    ///
    /// ```rust
    /// # use reqwest::Body;
    /// # fn run() -> Result<(), Box<std::error::Error>> {
    /// let s = "A stringy body";
    /// let body = Body::from(s);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<R: Read + Send + 'static>(reader: R) -> Body {
        Body {
            kind: Kind::Reader(Box::from(reader), None),
        }
    }

    /// Create a `Body` from a `Read` where the size is known in advance
    /// advance, but the data should not be loaded in full to memory. This will
    /// set the `Content-Length` header, and stream from the `Read`.
    ///
    /// ```rust
    /// # use std::fs::File;
    /// # use reqwest::Body;
    /// # fn run() -> Result<(), Box<std::error::Error>> {
    /// let file = File::open("a_large_file.txt")?;
    /// let file_size = file.metadata()?.len();
    /// let body = Body::sized(file, file_size);
    /// # Ok(())
    /// # }
    /// ```
    pub fn sized<R: Read + Send + 'static>(reader: R, len: u64) -> Body {
        Body {
            kind: Kind::Reader(Box::from(reader), Some(len)),
        }
    }
}


enum Kind {
    Reader(Box<Read + Send>, Option<u64>),
    Bytes(Bytes),
}

impl From<Vec<u8>> for Body {
    #[inline]
    fn from(v: Vec<u8>) -> Body {
        Body {
            kind: Kind::Bytes(v.into()),
        }
    }
}

impl From<String> for Body {
    #[inline]
    fn from(s: String) -> Body {
        s.into_bytes().into()
    }
}


impl From<&'static [u8]> for Body {
    #[inline]
    fn from(s: &'static [u8]) -> Body {
        Body {
            kind: Kind::Bytes(Bytes::from_static(s)),
        }
    }
}

impl From<&'static str> for Body {
    #[inline]
    fn from(s: &'static str) -> Body {
        s.as_bytes().into()
    }
}

impl From<File> for Body {
    #[inline]
    fn from(f: File) -> Body {
        let len = f.metadata().map(|m| m.len()).ok();
        Body {
            kind: Kind::Reader(Box::new(f), len),
        }
    }
}

impl fmt::Debug for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Kind::Reader(_, ref v) => f.debug_struct("Reader")
                .field("length", &DebugLength(v))
                .finish(),
            Kind::Bytes(ref v) => fmt::Debug::fmt(v, f),
        }
    }
}

struct DebugLength<'a>(&'a Option<u64>);

impl<'a> fmt::Debug for DebugLength<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Some(ref len) => fmt::Debug::fmt(len, f),
            None => f.write_str("Unknown"),
        }
    }
}


// pub(crate)

pub fn len(body: &Body) -> Option<u64> {
    match body.kind {
        Kind::Reader(_, len) => len,
        Kind::Bytes(ref bytes) => Some(bytes.len() as u64),
    }
}

pub enum Reader {
    Reader(Box<Read + Send>),
    Bytes(Cursor<Bytes>),
}

#[inline]
pub fn reader(body: Body) -> Reader {
    match body.kind {
        Kind::Reader(r, _) => Reader::Reader(r),
        Kind::Bytes(b) => Reader::Bytes(Cursor::new(b)),
    }
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Reader::Reader(ref mut rdr) => rdr.read(buf),
            Reader::Bytes(ref mut rdr) => rdr.read(buf),
        }
    }
}

pub struct Sender {
    body: (Box<Read + Send>, Option<u64>),
    tx: wait::WaitSink<::futures::sync::mpsc::Sender<hyper::Result<Chunk>>>,
}

impl Sender {
    pub fn send(self) -> ::Result<()> {
        use std::cmp;
        use bytes::{BufMut, BytesMut};

        let cap = cmp::min(self.body.1.unwrap_or(8192), 8192);
        let mut buf = BytesMut::with_capacity(cap as usize);
        let mut body = self.body.0;
        let mut tx = self.tx;
        loop {
            match body.read(unsafe { buf.bytes_mut() }) {
                Ok(0) => return Ok(()),
                Ok(n) => {
                    unsafe { buf.advance_mut(n); }
                    if let Err(e) = tx.send(Ok(buf.take().freeze().into())) {
                        if let wait::Waited::Err(_) = e {
                            let epipe = io::Error::new(io::ErrorKind::BrokenPipe, "broken pipe");
                            return Err(::error::from(epipe));
                        } else {
                            return Err(::error::timedout(None));
                        }
                    }
                    if buf.remaining_mut() == 0 {
                        buf.reserve(8192);
                    }
                }
                Err(e) => {
                    let ret = io::Error::new(e.kind(), e.to_string());
                    let _ = tx.send(Err(e.into()));
                    return Err(::error::from(ret));
                }
            }
        }
    }
}

#[inline]
pub fn async(body: Body) -> (Option<Sender>, async_impl::Body, Option<u64>) {
    match body.kind {
        Kind::Reader(read, len) => {
            let (tx, rx) = hyper::Body::pair();
            let tx = Sender {
                body: (read, len),
                tx: wait::sink(tx, None),
            };
            (Some(tx), async_impl::body::wrap(rx), len)
        },
        Kind::Bytes(chunk) => {
            let len = chunk.len() as u64;
            (None, async_impl::body::reusable(chunk), Some(len))
        }
    }
}

// useful for tests, but not publicly exposed
#[cfg(test)]
pub fn read_to_string(mut body: Body) -> io::Result<String> {
    let mut s = String::new();
    match body.kind {
            Kind::Reader(ref mut reader, _) => reader.read_to_string(&mut s),
            Kind::Bytes(ref mut bytes) => (&**bytes).read_to_string(&mut s),
        }
        .map(|_| s)
}

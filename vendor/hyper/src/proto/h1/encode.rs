//use std::cmp;
use std::fmt;

use bytes::{Buf, IntoBuf};
use bytes::buf::{Chain, Take};
use iovec::IoVec;

/// Encoders to handle different Transfer-Encodings.
#[derive(Debug, Clone)]
pub struct Encoder {
    kind: Kind,
}

#[derive(Debug)]
pub struct EncodedBuf<B> {
    kind: BufKind<B>,
}

#[derive(Debug)]
pub struct NotEof;

#[derive(Debug, PartialEq, Clone)]
enum Kind {
    /// An Encoder for when Transfer-Encoding includes `chunked`.
    Chunked,
    /// An Encoder for when Content-Length is set.
    ///
    /// Enforces that the body is not longer than the Content-Length header.
    Length(u64),
    /// An Encoder for when neither Content-Length nore Chunked encoding is set.
    ///
    /// This is mostly only used with HTTP/1.0 with a length. This kind requires
    /// the connection to be closed when the body is finished.
    Eof
}

#[derive(Debug)]
enum BufKind<B> {
    Exact(B),
    Limited(Take<B>),
    Chunked(Chain<ChunkSize, Chain<B, CrLf>>),
    ChunkedEnd(CrLf),
}

impl Encoder {
    pub fn chunked() -> Encoder {
        Encoder {
            kind: Kind::Chunked,
        }
    }

    pub fn length(len: u64) -> Encoder {
        Encoder {
            kind: Kind::Length(len),
        }
    }

    pub fn eof() -> Encoder {
        Encoder {
            kind: Kind::Eof,
        }
    }

    pub fn is_eof(&self) -> bool {
        match self.kind {
            Kind::Length(0) => true,
            _ => false
        }
    }

    pub fn end<B>(&self) -> Result<Option<EncodedBuf<B>>, NotEof> {
        match self.kind {
            Kind::Length(0) => Ok(None),
            Kind::Chunked => Ok(Some(EncodedBuf {
                kind: BufKind::ChunkedEnd(CrLf(b"0\r\n\r\n")),
            })),
            _ => Err(NotEof),
        }
    }

    pub fn encode<B>(&mut self, msg: B) -> EncodedBuf<B::Buf>
    where
        B: IntoBuf,
    {
        let msg = msg.into_buf();
        let len = msg.remaining();
        assert!(len > 0, "encode() called with empty buf");

        let buf = match self.kind {
            Kind::Chunked => {
                trace!("encoding chunked {}B", len);
                BufKind::Chunked(ChunkSize::new(len)
                    .chain(msg.chain(CrLf(b"\r\n"))))
            },
            Kind::Length(ref mut remaining) => {
                trace!("sized write, len = {}", len);
                if len as u64 > *remaining {
                    let limit = *remaining as usize;
                    *remaining = 0;
                    BufKind::Limited(msg.take(limit))
                } else {
                    *remaining -= len as u64;
                    BufKind::Exact(msg)
                }
            },
            Kind::Eof => {
                trace!("eof write {}B", len);
                BufKind::Exact(msg)
            }
        };
        EncodedBuf {
            kind: buf,
        }
    }
}

impl<B> Buf for EncodedBuf<B>
where
    B: Buf,
{
    #[inline]
    fn remaining(&self) -> usize {
        match self.kind {
            BufKind::Exact(ref b) => b.remaining(),
            BufKind::Limited(ref b) => b.remaining(),
            BufKind::Chunked(ref b) => b.remaining(),
            BufKind::ChunkedEnd(ref b) => b.remaining(),
        }
    }

    #[inline]
    fn bytes(&self) -> &[u8] {
        match self.kind {
            BufKind::Exact(ref b) => b.bytes(),
            BufKind::Limited(ref b) => b.bytes(),
            BufKind::Chunked(ref b) => b.bytes(),
            BufKind::ChunkedEnd(ref b) => b.bytes(),
        }
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        match self.kind {
            BufKind::Exact(ref mut b) => b.advance(cnt),
            BufKind::Limited(ref mut b) => b.advance(cnt),
            BufKind::Chunked(ref mut b) => b.advance(cnt),
            BufKind::ChunkedEnd(ref mut b) => b.advance(cnt),
        }
    }

    #[inline]
    fn bytes_vec<'t>(&'t self, dst: &mut [&'t IoVec]) -> usize {
        match self.kind {
            BufKind::Exact(ref b) => b.bytes_vec(dst),
            BufKind::Limited(ref b) => b.bytes_vec(dst),
            BufKind::Chunked(ref b) => b.bytes_vec(dst),
            BufKind::ChunkedEnd(ref b) => b.bytes_vec(dst),
        }
    }
}


#[cfg(target_pointer_width = "32")]
const USIZE_BYTES: usize = 4;

#[cfg(target_pointer_width = "64")]
const USIZE_BYTES: usize = 8;

// each byte will become 2 hex
const CHUNK_SIZE_MAX_BYTES: usize = USIZE_BYTES * 2;

#[derive(Clone, Copy)]
struct ChunkSize {
    bytes: [u8; CHUNK_SIZE_MAX_BYTES + 2],
    pos: u8,
    len: u8,
}

impl ChunkSize {
    fn new(len: usize) -> ChunkSize {
        use std::fmt::Write;
        let mut size = ChunkSize {
            bytes: [0; CHUNK_SIZE_MAX_BYTES + 2],
            pos: 0,
            len: 0,
        };
        write!(&mut size, "{:X}\r\n", len)
            .expect("CHUNK_SIZE_MAX_BYTES should fit any usize");
        size
    }
}

impl Buf for ChunkSize {
    #[inline]
    fn remaining(&self) -> usize {
        (self.len - self.pos).into()
    }

    #[inline]
    fn bytes(&self) -> &[u8] {
        &self.bytes[self.pos.into() .. self.len.into()]
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        assert!(cnt <= self.remaining());
        self.pos += cnt as u8; // just asserted cnt fits in u8
    }
}

impl fmt::Debug for ChunkSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ChunkSize")
            .field("bytes", &&self.bytes[..self.len.into()])
            .field("pos", &self.pos)
            .finish()
    }
}

impl fmt::Write for ChunkSize {
    fn write_str(&mut self, num: &str) -> fmt::Result {
        use std::io::Write;
        (&mut self.bytes[self.len.into()..]).write(num.as_bytes())
            .expect("&mut [u8].write() cannot error");
        self.len += num.len() as u8; // safe because bytes is never bigger than 256
        Ok(())
    }
}

#[derive(Debug)]
struct CrLf(&'static [u8]);

impl Buf for CrLf {
    #[inline]
    fn remaining(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn bytes(&self) -> &[u8] {
        self.0
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        self.0 = &self.0[cnt..];
    }

    #[inline]
    fn bytes_vec<'t>(&'t self, dst: &mut [&'t IoVec]) -> usize {
        if dst.is_empty() {
            return 0;
        } else {
            dst[0] = self.0.into();
            return 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::{BufMut};

    use super::super::io::Cursor;
    use super::Encoder;

    #[test]
    fn chunked() {
        let mut encoder = Encoder::chunked();
        let mut dst = Vec::new();

        let msg1 = b"foo bar".as_ref();
        let buf1 = encoder.encode(msg1);
        dst.put(buf1);
        assert_eq!(dst, b"7\r\nfoo bar\r\n");

        let msg2 = b"baz quux herp".as_ref();
        let buf2 = encoder.encode(msg2);
        dst.put(buf2);

        assert_eq!(dst, b"7\r\nfoo bar\r\nD\r\nbaz quux herp\r\n");

        let end = encoder.end::<Cursor<Vec<u8>>>().unwrap().unwrap();
        dst.put(end);

        assert_eq!(dst, b"7\r\nfoo bar\r\nD\r\nbaz quux herp\r\n0\r\n\r\n".as_ref());
    }

    #[test]
    fn length() {
        let max_len = 8;
        let mut encoder = Encoder::length(max_len as u64);
        let mut dst = Vec::new();


        let msg1 = b"foo bar".as_ref();
        let buf1 = encoder.encode(msg1);
        dst.put(buf1);


        assert_eq!(dst, b"foo bar");
        assert!(!encoder.is_eof());
        encoder.end::<()>().unwrap_err();

        let msg2 = b"baz".as_ref();
        let buf2 = encoder.encode(msg2);
        dst.put(buf2);

        assert_eq!(dst.len(), max_len);
        assert_eq!(dst, b"foo barb");
        assert!(encoder.is_eof());
        assert!(encoder.end::<()>().unwrap().is_none());
    }

    #[test]
    fn eof() {
        let mut encoder = Encoder::eof();
        let mut dst = Vec::new();


        let msg1 = b"foo bar".as_ref();
        let buf1 = encoder.encode(msg1);
        dst.put(buf1);


        assert_eq!(dst, b"foo bar");
        assert!(!encoder.is_eof());
        encoder.end::<()>().unwrap_err();

        let msg2 = b"baz".as_ref();
        let buf2 = encoder.encode(msg2);
        dst.put(buf2);

        assert_eq!(dst, b"foo barbaz");
        assert!(!encoder.is_eof());
        encoder.end::<()>().unwrap_err();
    }
}

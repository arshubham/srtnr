use std::io;
use std::cmp;
use byteorder::LittleEndian;
use byteorder::WriteBytesExt;

use bit;
use lz77;
use finish::{Complete, Finish};
use super::symbol;
use super::BlockType;

/// The default size of a DEFLATE block.
pub const DEFAULT_BLOCK_SIZE: usize = 1024 * 1024;

const MAX_NON_COMPRESSED_BLOCK_SIZE: usize = 0xFFFF;

/// Options for a DEFLATE encoder.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EncodeOptions<E = lz77::DefaultLz77Encoder> {
    block_size: usize,
    dynamic_huffman: bool,
    lz77: Option<E>,
}
impl Default for EncodeOptions<lz77::DefaultLz77Encoder> {
    fn default() -> Self {
        Self::new()
    }
}
impl EncodeOptions<lz77::DefaultLz77Encoder> {
    /// Makes a default instance.
    ///
    /// # Examples
    /// ```
    /// use libflate::deflate::{Encoder, EncodeOptions};
    ///
    /// let options = EncodeOptions::new();
    /// let encoder = Encoder::with_options(Vec::new(), options);
    /// ```
    pub fn new() -> Self {
        EncodeOptions {
            block_size: DEFAULT_BLOCK_SIZE,
            dynamic_huffman: true,
            lz77: Some(lz77::DefaultLz77Encoder::new()),
        }
    }
}
impl<E> EncodeOptions<E>
where
    E: lz77::Lz77Encode,
{
    /// Specifies the LZ77 encoder used to compress input data.
    ///
    /// # Example
    /// ```
    /// use libflate::lz77::DefaultLz77Encoder;
    /// use libflate::deflate::{Encoder, EncodeOptions};
    ///
    /// let options = EncodeOptions::with_lz77(DefaultLz77Encoder::new());
    /// let encoder = Encoder::with_options(Vec::new(), options);
    /// ```
    pub fn with_lz77(lz77: E) -> Self {
        EncodeOptions {
            block_size: DEFAULT_BLOCK_SIZE,
            dynamic_huffman: true,
            lz77: Some(lz77),
        }
    }

    /// Disables LZ77 compression.
    ///
    /// # Example
    /// ```
    /// use libflate::lz77::DefaultLz77Encoder;
    /// use libflate::deflate::{Encoder, EncodeOptions};
    ///
    /// let options = EncodeOptions::new().no_compression();
    /// let encoder = Encoder::with_options(Vec::new(), options);
    /// ```
    pub fn no_compression(mut self) -> Self {
        self.lz77 = None;
        self
    }

    /// Specifies the hint of the size of a DEFLATE block.
    ///
    /// The default value is `DEFAULT_BLOCK_SIZE`.
    ///
    /// # Example
    /// ```
    /// use libflate::deflate::{Encoder, EncodeOptions};
    ///
    /// let options = EncodeOptions::new().block_size(512 * 1024);
    /// let encoder = Encoder::with_options(Vec::new(), options);
    /// ```
    pub fn block_size(mut self, size: usize) -> Self {
        self.block_size = size;
        self
    }

    /// Specifies to compress with fixed huffman codes.
    ///
    /// # Example
    /// ```
    /// use libflate::deflate::{Encoder, EncodeOptions};
    ///
    /// let options = EncodeOptions::new().fixed_huffman_codes();
    /// let encoder = Encoder::with_options(Vec::new(), options);
    /// ```
    pub fn fixed_huffman_codes(mut self) -> Self {
        self.dynamic_huffman = false;
        self
    }

    fn get_block_type(&self) -> BlockType {
        if self.lz77.is_none() {
            BlockType::Raw
        } else if self.dynamic_huffman {
            BlockType::Dynamic
        } else {
            BlockType::Fixed
        }
    }
    fn get_block_size(&self) -> usize {
        if self.lz77.is_none() {
            cmp::min(self.block_size, MAX_NON_COMPRESSED_BLOCK_SIZE)
        } else {
            self.block_size
        }
    }
}

/// DEFLATE encoder.
#[derive(Debug)]
pub struct Encoder<W, E = lz77::DefaultLz77Encoder> {
    writer: bit::BitWriter<W>,
    block: Block<E>,
}
impl<W> Encoder<W, lz77::DefaultLz77Encoder>
where
    W: io::Write,
{
    /// Makes a new encoder instance.
    ///
    /// Encoded DEFLATE stream is written to `inner`.
    ///
    /// # Examples
    /// ```
    /// use std::io::Write;
    /// use libflate::deflate::Encoder;
    ///
    /// let mut encoder = Encoder::new(Vec::new());
    /// encoder.write_all(b"Hello World!").unwrap();
    ///
    /// assert_eq!(encoder.finish().into_result().unwrap(),
    ///            [5, 128, 65, 9, 0, 0, 8, 3, 171, 104, 27, 27, 88, 64, 127,
    ///             7, 131, 245, 127, 140, 121, 80, 173, 204, 117, 0]);
    /// ```
    pub fn new(inner: W) -> Self {
        Self::with_options(inner, EncodeOptions::default())
    }
}
impl<W, E> Encoder<W, E>
where
    W: io::Write,
    E: lz77::Lz77Encode,
{
    /// Makes a new encoder instance with specified options.
    ///
    /// Encoded DEFLATE stream is written to `inner`.
    ///
    /// # Examples
    /// ```
    /// use std::io::Write;
    /// use libflate::deflate::{Encoder, EncodeOptions};
    ///
    /// let options = EncodeOptions::new().no_compression();
    /// let mut encoder = Encoder::with_options(Vec::new(), options);
    /// encoder.write_all(b"Hello World!").unwrap();
    ///
    /// assert_eq!(encoder.finish().into_result().unwrap(),
    ///            [1, 12, 0, 243, 255, 72, 101, 108, 108, 111, 32, 87, 111,
    ///             114, 108, 100, 33]);
    /// ```
    pub fn with_options(inner: W, options: EncodeOptions<E>) -> Self {
        Encoder {
            writer: bit::BitWriter::new(inner),
            block: Block::new(options),
        }
    }

    /// Flushes internal buffer and returns the inner stream.
    ///
    /// # Examples
    /// ```
    /// use std::io::Write;
    /// use libflate::deflate::Encoder;
    ///
    /// let mut encoder = Encoder::new(Vec::new());
    /// encoder.write_all(b"Hello World!").unwrap();
    ///
    /// assert_eq!(encoder.finish().into_result().unwrap(),
    ///            [5, 128, 65, 9, 0, 0, 8, 3, 171, 104, 27, 27, 88, 64, 127,
    ///             7, 131, 245, 127, 140, 121, 80, 173, 204, 117, 0]);
    /// ```
    pub fn finish(mut self) -> Finish<W, io::Error> {
        match self.block.finish(&mut self.writer) {
            Ok(_) => Finish::new(self.writer.into_inner(), None),
            Err(e) => Finish::new(self.writer.into_inner(), Some(e)),
        }
    }

    /// Returns the immutable reference to the inner stream.
    pub fn as_inner_ref(&self) -> &W {
        self.writer.as_inner_ref()
    }

    /// Returns the mutable reference to the inner stream.
    pub fn as_inner_mut(&mut self) -> &mut W {
        self.writer.as_inner_mut()
    }

    /// Unwraps the `Encoder`, returning the inner stream.
    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }
}
impl<W, E> io::Write for Encoder<W, E>
where
    W: io::Write,
    E: lz77::Lz77Encode,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.block.write(&mut self.writer, buf)?;
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.as_inner_mut().flush()
    }
}
impl<W, E> Complete for Encoder<W, E>
where
    W: io::Write,
    E: lz77::Lz77Encode,
{
    fn complete(self) -> io::Result<()> {
        self.finish().into_result().map(|_| ())
    }
}

#[derive(Debug)]
struct Block<E> {
    block_type: BlockType,
    block_size: usize,
    block_buf: BlockBuf<E>,
}
impl<E> Block<E>
where
    E: lz77::Lz77Encode,
{
    fn new(options: EncodeOptions<E>) -> Self {
        Block {
            block_type: options.get_block_type(),
            block_size: options.get_block_size(),
            block_buf: BlockBuf::new(options.lz77, options.dynamic_huffman),
        }
    }
    fn write<W>(&mut self, writer: &mut bit::BitWriter<W>, buf: &[u8]) -> io::Result<()>
    where
        W: io::Write,
    {
        self.block_buf.append(buf);
        while self.block_buf.len() >= self.block_size {
            writer.write_bit(false)?;
            writer.write_bits(2, self.block_type as u16)?;
            self.block_buf.flush(writer)?;
        }
        Ok(())
    }
    fn finish<W>(mut self, writer: &mut bit::BitWriter<W>) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_bit(true)?;
        writer.write_bits(2, self.block_type as u16)?;
        self.block_buf.flush(writer)?;
        writer.flush()?;
        Ok(())
    }
}

#[derive(Debug)]
enum BlockBuf<E> {
    Raw(RawBuf),
    Fixed(CompressBuf<symbol::FixedHuffmanCodec, E>),
    Dynamic(CompressBuf<symbol::DynamicHuffmanCodec, E>),
}
impl<E> BlockBuf<E>
where
    E: lz77::Lz77Encode,
{
    fn new(lz77: Option<E>, dynamic: bool) -> Self {
        if let Some(lz77) = lz77 {
            if dynamic {
                BlockBuf::Dynamic(CompressBuf::new(symbol::DynamicHuffmanCodec, lz77))
            } else {
                BlockBuf::Fixed(CompressBuf::new(symbol::FixedHuffmanCodec, lz77))
            }
        } else {
            BlockBuf::Raw(RawBuf::new())
        }
    }
    fn append(&mut self, buf: &[u8]) {
        match *self {
            BlockBuf::Raw(ref mut b) => b.append(buf),
            BlockBuf::Fixed(ref mut b) => b.append(buf),
            BlockBuf::Dynamic(ref mut b) => b.append(buf),
        }
    }
    fn len(&self) -> usize {
        match *self {
            BlockBuf::Raw(ref b) => b.len(),
            BlockBuf::Fixed(ref b) => b.len(),
            BlockBuf::Dynamic(ref b) => b.len(),
        }
    }
    fn flush<W>(&mut self, writer: &mut bit::BitWriter<W>) -> io::Result<()>
    where
        W: io::Write,
    {
        match *self {
            BlockBuf::Raw(ref mut b) => b.flush(writer),
            BlockBuf::Fixed(ref mut b) => b.flush(writer),
            BlockBuf::Dynamic(ref mut b) => b.flush(writer),
        }
    }
}

#[derive(Debug)]
struct RawBuf {
    buf: Vec<u8>,
}
impl RawBuf {
    fn new() -> Self {
        RawBuf { buf: Vec::new() }
    }
    fn append(&mut self, buf: &[u8]) {
        self.buf.extend(buf);
    }
    fn len(&self) -> usize {
        self.buf.len()
    }
    fn flush<W>(&mut self, writer: &mut bit::BitWriter<W>) -> io::Result<()>
    where
        W: io::Write,
    {
        let size = cmp::min(self.buf.len(), MAX_NON_COMPRESSED_BLOCK_SIZE);
        writer.flush()?;
        writer
            .as_inner_mut()
            .write_u16::<LittleEndian>(size as u16)?;
        writer
            .as_inner_mut()
            .write_u16::<LittleEndian>(!size as u16)?;
        writer.as_inner_mut().write_all(&self.buf[..size])?;
        self.buf.drain(0..size);
        Ok(())
    }
}

#[derive(Debug)]
struct CompressBuf<H, E> {
    huffman: H,
    lz77: E,
    buf: Vec<symbol::Symbol>,
    original_size: usize,
}
impl<H, E> CompressBuf<H, E>
where
    H: symbol::HuffmanCodec,
    E: lz77::Lz77Encode,
{
    fn new(huffman: H, lz77: E) -> Self {
        CompressBuf {
            huffman: huffman,
            lz77: lz77,
            buf: Vec::new(),
            original_size: 0,
        }
    }
    fn append(&mut self, buf: &[u8]) {
        self.original_size += buf.len();
        self.lz77.encode(buf, &mut self.buf);
    }
    fn len(&self) -> usize {
        self.original_size
    }
    fn flush<W>(&mut self, writer: &mut bit::BitWriter<W>) -> io::Result<()>
    where
        W: io::Write,
    {
        self.lz77.flush(&mut self.buf);
        self.buf.push(symbol::Symbol::EndOfBlock);
        let symbol_encoder = self.huffman.build(&self.buf);
        self.huffman.save(writer, &symbol_encoder)?;
        for s in self.buf.drain(..) {
            symbol_encoder.encode(writer, &s)?;
        }
        self.original_size = 0;
        Ok(())
    }
}

impl lz77::Sink for Vec<symbol::Symbol> {
    fn consume(&mut self, code: lz77::Code) {
        let symbol = match code {
            lz77::Code::Literal(b) => symbol::Symbol::Literal(b),
            lz77::Code::Pointer {
                length,
                backward_distance,
            } => symbol::Symbol::Share {
                length: length,
                distance: backward_distance,
            },
        };
        self.push(symbol);
    }
}

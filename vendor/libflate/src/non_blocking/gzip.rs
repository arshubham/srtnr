//! The encoder and decoder of the GZIP format.
//!
//! The GZIP format is defined in [RFC-1952](https://tools.ietf.org/html/rfc1952).
//!
//! # Examples
//! ```
//! use std::io::{self, Read};
//! use libflate::gzip::Encoder;
//! use libflate::non_blocking::gzip::Decoder;
//!
//! // Encoding
//! let mut encoder = Encoder::new(Vec::new()).unwrap();
//! io::copy(&mut &b"Hello World!"[..], &mut encoder).unwrap();
//! let encoded_data = encoder.finish().into_result().unwrap();
//!
//! // Decoding
//! let mut decoder = Decoder::new(&encoded_data[..]);
//! let mut decoded_data = Vec::new();
//! decoder.read_to_end(&mut decoded_data).unwrap();
//!
//! assert_eq!(decoded_data, b"Hello World!");
//! ```
use std::io::{self, Read};

use checksum;
use gzip::{Header, Trailer};
use non_blocking::deflate;

/// GZIP decoder which supports non-blocking I/O.
#[derive(Debug)]
pub struct Decoder<R> {
    header: Option<Header>,
    reader: deflate::Decoder<R>,
    crc32: checksum::Crc32,
    eos: bool,
}
impl<R: Read> Decoder<R> {
    /// Makes a new decoder instance.
    ///
    /// `inner` is to be decoded GZIP stream.
    ///
    /// # Examples
    /// ```
    /// use std::io::Read;
    /// use libflate::non_blocking::gzip::Decoder;
    ///
    /// let encoded_data = [31, 139, 8, 0, 123, 0, 0, 0, 0, 3, 1, 12, 0, 243, 255,
    ///                     72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33,
    ///                     163, 28, 41, 28, 12, 0, 0, 0];
    ///
    /// let mut decoder = Decoder::new(&encoded_data[..]);
    /// let mut buf = Vec::new();
    /// decoder.read_to_end(&mut buf).unwrap();
    ///
    /// assert_eq!(buf, b"Hello World!");
    /// ```
    pub fn new(inner: R) -> Self {
        Decoder {
            header: None,
            reader: deflate::Decoder::new(inner),
            crc32: checksum::Crc32::new(),
            eos: false,
        }
    }

    /// Returns the header of the GZIP stream.
    ///
    /// # Examples
    /// ```
    /// use libflate::gzip::Os;
    /// use libflate::non_blocking::gzip::Decoder;
    ///
    /// let encoded_data = [31, 139, 8, 0, 123, 0, 0, 0, 0, 3, 1, 12, 0, 243, 255,
    ///                     72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33,
    ///                     163, 28, 41, 28, 12, 0, 0, 0];
    ///
    /// let mut decoder = Decoder::new(&encoded_data[..]);
    /// assert_eq!(decoder.header().unwrap().os(), Os::Unix);
    /// ```
    pub fn header(&mut self) -> io::Result<&Header> {
        if let Some(ref header) = self.header {
            Ok(header)
        } else {
            let header = self.reader
                .bit_reader_mut()
                .transaction(|r| Header::read_from(r.as_inner_mut()))?;
            self.header = Some(header);
            self.header()
        }
    }

    /// Unwraps this `Decoder`, returning the underlying reader.
    ///
    /// # Examples
    /// ```
    /// use std::io::Cursor;
    /// use libflate::non_blocking::gzip::Decoder;
    ///
    /// let encoded_data = [31, 139, 8, 0, 123, 0, 0, 0, 0, 3, 1, 12, 0, 243, 255,
    ///                     72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33,
    ///                     163, 28, 41, 28, 12, 0, 0, 0];
    ///
    /// let decoder = Decoder::new(Cursor::new(&encoded_data[..]));
    /// assert_eq!(decoder.into_inner().into_inner(), &encoded_data[..]);
    /// ```
    pub fn into_inner(self) -> R {
        self.reader.into_inner()
    }
}
impl<R: Read> Read for Decoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.header.is_none() {
            self.header()?;
        }
        if self.eos {
            Ok(0)
        } else {
            let read_size = self.reader.read(buf)?;
            self.crc32.update(&buf[..read_size]);
            if read_size == 0 {
                let trailer = self.reader
                    .bit_reader_mut()
                    .transaction(|r| Trailer::read_from(r.as_inner_mut()))?;
                self.eos = true;
                if trailer.crc32() != self.crc32.value() {
                    Err(invalid_data_error!(
                        "CRC32 mismatched: value={}, expected={}",
                        self.crc32.value(),
                        trailer.crc32()
                    ))
                } else {
                    Ok(0)
                }
            } else {
                Ok(read_size)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::io;
    use gzip::Encoder;
    use util::{nb_read_to_end, WouldBlockReader};
    use super::*;

    fn decode_all(buf: &[u8]) -> io::Result<Vec<u8>> {
        let decoder = Decoder::new(WouldBlockReader::new(buf));
        nb_read_to_end(decoder)
    }

    #[test]
    fn encode_works() {
        let plain = b"Hello World! Hello GZIP!!";
        let mut encoder = Encoder::new(Vec::new()).unwrap();
        io::copy(&mut &plain[..], &mut encoder).unwrap();
        let encoded = encoder.finish().into_result().unwrap();
        assert_eq!(decode_all(&encoded).unwrap(), plain);
    }
}

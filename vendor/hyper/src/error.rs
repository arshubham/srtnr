//! Error and Result module.
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

use httparse;

pub use uri::UriError;

use self::Error::{
    Method,
    Uri,
    Version,
    Header,
    Status,
    Timeout,
    Upgrade,
    Cancel,
    Io,
    TooLarge,
    Incomplete,
    Utf8
};

/// Result type often returned from methods that can have hyper `Error`s.
pub type Result<T> = ::std::result::Result<T, Error>;

/// A set of errors that can occur parsing HTTP streams.
#[derive(Debug)]
pub enum Error {
    /// An invalid `Method`, such as `GE,T`.
    Method,
    /// An invalid `Uri`, such as `exam ple.domain`.
    Uri(UriError),
    /// An invalid `HttpVersion`, such as `HTP/1.1`
    Version,
    /// An invalid `Header`.
    Header,
    /// A message head is too large to be reasonable.
    TooLarge,
    /// A message reached EOF, but is not complete.
    Incomplete,
    /// An invalid `Status`, such as `1337 ELITE`.
    Status,
    /// A timeout occurred waiting for an IO event.
    Timeout,
    /// A protocol upgrade was encountered, but not yet supported in hyper.
    Upgrade,
    /// A pending item was dropped before ever being processed.
    Cancel(Canceled),
    /// An `io::Error` that occurred while trying to read or write to a network stream.
    Io(IoError),
    /// Parsing a field as string failed
    Utf8(Utf8Error),

    #[doc(hidden)]
    __Nonexhaustive(Void)
}

impl Error {
    pub(crate) fn new_canceled() -> Error {
        Error::Cancel(Canceled {
            _inner: (),
        })
    }
}

/// A pending item was dropped before ever being processed.
///
/// For example, a `Request` could be queued in the `Client`, *just*
/// as the related connection gets closed by the remote. In that case,
/// when the connection drops, the pending response future will be
/// fulfilled with this error, signaling the `Request` was never started.
pub struct Canceled {
    // maybe in the future this contains an optional value of
    // what was canceled?
    _inner: (),
}

impl Canceled {
    fn description(&self) -> &str {
        "an operation was canceled internally before starting"
    }
}

impl fmt::Debug for Canceled {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Canceled")
            .finish()
    }
}

impl fmt::Display for Canceled {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(self.description())
    }
}

#[doc(hidden)]
pub struct Void(());

impl fmt::Debug for Void {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Uri(ref e) => fmt::Display::fmt(e, f),
            Io(ref e) => fmt::Display::fmt(e, f),
            Utf8(ref e) => fmt::Display::fmt(e, f),
            ref e => f.write_str(e.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Method => "invalid Method specified",
            Version => "invalid HTTP version specified",
            Header => "invalid Header provided",
            TooLarge => "message head is too large",
            Status => "invalid Status provided",
            Incomplete => "message is incomplete",
            Timeout => "timeout",
            Upgrade => "unsupported protocol upgrade",
            Cancel(ref e) => e.description(),
            Uri(ref e) => e.description(),
            Io(ref e) => e.description(),
            Utf8(ref e) => e.description(),
            Error::__Nonexhaustive(..) =>  unreachable!(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Io(ref error) => Some(error),
            Uri(ref error) => Some(error),
            Utf8(ref error) => Some(error),
            Error::__Nonexhaustive(..) =>  unreachable!(),
            _ => None,
        }
    }
}

impl From<UriError> for Error {
    fn from(err: UriError) -> Error {
        Uri(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Io(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Utf8(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Utf8(err.utf8_error())
    }
}

impl From<httparse::Error> for Error {
    fn from(err: httparse::Error) -> Error {
        match err {
            httparse::Error::HeaderName |
            httparse::Error::HeaderValue |
            httparse::Error::NewLine |
            httparse::Error::Token => Header,
            httparse::Error::Status => Status,
            httparse::Error::TooManyHeaders => TooLarge,
            httparse::Error::Version => Version,
        }
    }
}

#[doc(hidden)]
trait AssertSendSync: Send + Sync + 'static {}
#[doc(hidden)]
impl AssertSendSync for Error {}

#[cfg(test)]
mod tests {
    use std::error::Error as StdError;
    use std::io;
    use httparse;
    use super::Error;
    use super::Error::*;

    #[test]
    fn test_cause() {
        let orig = io::Error::new(io::ErrorKind::Other, "other");
        let desc = orig.description().to_owned();
        let e = Io(orig);
        assert_eq!(e.cause().unwrap().description(), desc);
    }

    macro_rules! from {
        ($from:expr => $error:pat) => {
            match Error::from($from) {
                e @ $error => {
                    assert!(e.description().len() >= 5);
                } ,
                e => panic!("{:?}", e)
            }
        }
    }

    macro_rules! from_and_cause {
        ($from:expr => $error:pat) => {
            match Error::from($from) {
                e @ $error => {
                    let desc = e.cause().unwrap().description();
                    assert_eq!(desc, $from.description().to_owned());
                    assert_eq!(desc, e.description());
                },
                _ => panic!("{:?}", $from)
            }
        }
    }

    #[test]
    fn test_from() {

        from_and_cause!(io::Error::new(io::ErrorKind::Other, "other") => Io(..));

        from!(httparse::Error::HeaderName => Header);
        from!(httparse::Error::HeaderName => Header);
        from!(httparse::Error::HeaderValue => Header);
        from!(httparse::Error::NewLine => Header);
        from!(httparse::Error::Status => Status);
        from!(httparse::Error::Token => Header);
        from!(httparse::Error::TooManyHeaders => TooLarge);
        from!(httparse::Error::Version => Version);
    }
}

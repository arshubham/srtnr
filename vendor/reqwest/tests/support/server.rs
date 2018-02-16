//! A server builder helper for the integration tests.

use std::io::{Read, Write};
use std::net;
use std::time::Duration;
use std::thread;

pub struct Server {
    addr: net::SocketAddr,
}

impl Server {
    pub fn addr(&self) -> net::SocketAddr {
        self.addr
    }
}

#[derive(Default)]
pub struct Txn {
    pub request: Vec<u8>,
    pub response: Vec<u8>,

    pub read_timeout: Option<Duration>,
    pub response_timeout: Option<Duration>,
    pub write_timeout: Option<Duration>,
    pub chunk_size: Option<usize>,
}

static DEFAULT_USER_AGENT: &'static str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn spawn(txns: Vec<Txn>) -> Server {
    let listener = net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    thread::spawn(
        move || for txn in txns {
            let mut expected = txn.request;
            let reply = txn.response;
            let (mut socket, _addr) = listener.accept().unwrap();

            socket.set_read_timeout(Some(Duration::from_secs(5))).unwrap();

            replace_expected_vars(&mut expected, addr.to_string().as_ref(), DEFAULT_USER_AGENT.as_ref());

            if let Some(dur) = txn.read_timeout {
                thread::park_timeout(dur);
            }

            let mut buf = [0; 4096];
            assert!(buf.len() >= expected.len());

            let mut n = 0;
            while n < expected.len() {
                match socket.read(&mut buf[n..]) {
                    Ok(0) | Err(_) => break,
                    Ok(nread) => n += nread,
                }
            }

            match (::std::str::from_utf8(&expected), ::std::str::from_utf8(&buf[..n])) {
                (Ok(expected), Ok(received)) => assert_eq!(expected, received),
                _ => assert_eq!(expected, &buf[..n]),
            }

            if let Some(dur) = txn.response_timeout {
                thread::park_timeout(dur);
            }

            if let Some(dur) = txn.write_timeout {
                let headers_end = b"\r\n\r\n";
                let headers_end = reply.windows(headers_end.len()).position(|w| w == headers_end).unwrap() + 4;
                socket.write_all(&reply[..headers_end]).unwrap();

                let body = &reply[headers_end..];

                if let Some(chunk_size) = txn.chunk_size {
                    for content in body.chunks(chunk_size) {
                        thread::park_timeout(dur);
                        socket.write_all(&content).unwrap();
                    }
                } else {
                    thread::park_timeout(dur);
                    socket.write_all(&body).unwrap();
                }
            } else {
                socket.write_all(&reply).unwrap();
            }
        }
    );

    Server {
        addr: addr,
    }
}

fn replace_expected_vars(bytes: &mut Vec<u8>, host: &[u8], ua: &[u8]) {
    // plenty horrible, but these are just tests, and gets the job done
    let mut index = 0;
    loop {
        if index == bytes.len() {
            return;
        }

        for b in (&bytes[index..]).iter() {
            index += 1;
            if *b == b'$' {
                break;
            }
        }

        let has_host = (&bytes[index..]).starts_with(b"HOST");
        if has_host {
            bytes.drain(index - 1..index + 4);
            for (i, b) in host.iter().enumerate() {
                bytes.insert(index - 1 + i, *b);
            }
        } else {
            let has_ua = (&bytes[index..]).starts_with(b"USERAGENT");
            if has_ua {
                bytes.drain(index - 1..index + 9);
                for (i, b) in ua.iter().enumerate() {
                    bytes.insert(index - 1 + i, *b);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! server {
    ($(request: $req:expr, response: $res:expr),*) => ({
        server!($(request: $req, response: $res;)*)
    });
    ($($($f:ident: $v:expr),*);*) => ({
        let txns = vec![
            $(__internal__txn! {
                $($f: $v,)*
            }),*
        ];
        ::support::server::spawn(txns)
    })
}

#[macro_export]
macro_rules! __internal__txn {
    ($($field:ident: $val:expr,)*) => (
        ::support::server::Txn {
            $( $field: __internal__prop!($field: $val), )*
            .. Default::default()
        }
    )
}


#[macro_export]
macro_rules! __internal__prop {
    (request: $val:expr) => (
        From::from(&$val[..])
    );
    (response: $val:expr) => (
        From::from(&$val[..])
    );
    ($field:ident: $val:expr) => (
        From::from($val)
    )
}

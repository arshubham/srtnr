#![deny(warnings)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/reqwest/0.8.5")]

//! # reqwest
//!
//! The `reqwest` crate provides a convenient, higher-level HTTP Client.
//!
//! It handles many of the things that most people just expect an HTTP client
//! to do for them.
//!
//! - Uses system-native TLS
//! - Plain bodies, JSON, urlencoded, multipart
//! - Customizable redirect policy
//! - Proxies
//! - Cookies (only rudimentary support, full support is TODO)
//!
//! The rudimentary cookie support means that the cookies need to be manually
//! configured for every single request. In other words, there's no cookie jar
//! support as of now. The tracking issue for this feature is available
//! [on GitHub][cookiejar_issue].
//!
//! The `reqwest::Client` is synchronous, making it a great fit for
//! applications that only require a few HTTP requests, and wish to handle
//! them synchronously.
//!
//! ## Making a GET request
//!
//! For a single request, you can use the [`get`][get] shortcut method.
//!
//! ```rust
//! # use reqwest::{Error, Response};
//!
//! # fn run() -> Result<(), Error> {
//! let text = reqwest::get("https://www.rust-lang.org")?
//!     .text()?;
//!
//! println!("body = {:?}", body);
//! # Ok(())
//! # }
//! ```
//!
//! Additionally, reqwest's [`Response`][response] struct implements Rust's
//! `Read` trait, so many useful standard library and third party crates will
//! have convenience methods that take a `Response` anywhere `T: Read` is
//! acceptable.
//!
//! **NOTE**: If you plan to perform multiple requests, it is best to create a
//! [`Client`][client] and reuse it, taking advantage of keep-alive connection
//! pooling.
//!
//! ## Making POST requests (or setting request bodies)
//!
//! There are several ways you can set the body of a request. The basic one is
//! by using the `body()` method of a [`RequestBuilder`][builder]. This lets you set the
//! exact raw bytes of what the body should be. It accepts various types,
//! including `String`, `Vec<u8>`, and `File`. If you wish to pass a custom
//! Reader, you can use the `reqwest::Body::new()` constructor.
//!
//! ```rust
//! # use reqwest::Error;
//! #
//! # fn run() -> Result<(), Error> {
//! let client = reqwest::Client::new();
//! let res = client.post("http://httpbin.org/post")
//!     .body("the exact body that is sent")
//!     .send()?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Forms
//!
//! It's very common to want to send form data in a request body. This can be
//! done with any type that can be serialized into form data.
//!
//! This can be an array of tuples, or a `HashMap`, or a custom type that
//! implements [`Serialize`][serde].
//!
//! ```rust
//! # use reqwest::Error;
//! #
//! # fn run() -> Result<(), Error> {
//! // This will POST a body of `foo=bar&baz=quux`
//! let params = [("foo", "bar"), ("baz", "quux")];
//! let client = reqwest::Client::new();
//! let res = client.post("http://httpbin.org/post")
//!     .form(&params)
//!     .send()?;
//! # Ok(())
//! # }
//! ```
//!
//! ### JSON
//!
//! There is also a `json` method helper on the [`RequestBuilder`][builder] that works in
//! a similar fashion the `form` method. It can take any value that can be
//! serialized into JSON.
//!
//! ```rust
//! # use reqwest::Error;
//! # use std::collections::HashMap;
//! #
//! # fn run() -> Result<(), Error> {
//! // This will POST a body of `{"lang":"rust","body":"json"}`
//! let mut map = HashMap::new();
//! map.insert("lang", "rust");
//! map.insert("body", "json");
//!
//! let client = reqwest::Client::new();
//! let res = client.post("http://httpbin.org/post")
//!     .json(&map)
//!     .send()?;
//! # Ok(())
//! # }
//! ```
//!
//! [hyper]: http://hyper.rs
//! [client]: ./struct.Client.html
//! [response]: ./struct.Response.html
//! [get]: ./fn.get.html
//! [builder]: ./struct.RequestBuilder.html
//! [serde]: http://serde.rs
//! [cookiejar_issue]: https://github.com/seanmonstar/reqwest/issues/14

extern crate bytes;
extern crate encoding_rs;
#[macro_use]
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate log;
extern crate libflate;
extern crate mime_guess;
extern crate native_tls;
extern crate serde;
#[cfg(test)]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_tls;
extern crate url;
extern crate uuid;

pub use hyper::header;
pub use hyper::mime;
pub use hyper::Method;
pub use hyper::StatusCode;
pub use url::Url;
pub use url::ParseError as UrlError;

pub use self::client::{Client, ClientBuilder};
pub use self::error::{Error, Result};
pub use self::body::Body;
pub use self::into_url::IntoUrl;
pub use self::proxy::Proxy;
pub use self::redirect::{RedirectAction, RedirectAttempt, RedirectPolicy};
pub use self::request::{Request, RequestBuilder};
pub use self::response::Response;
pub use self::tls::{Certificate, Identity};


// this module must be first because of the `try_` macro
#[macro_use]
mod error;

mod async_impl;
mod connect;
mod body;
mod client;
mod into_url;
mod multipart_;
mod proxy;
mod redirect;
mod request;
mod response;
mod tls;
mod wait;

pub mod multipart;

/// A set of unstable functionality.
///
/// This module is only available when the `unstable` [feature][] is enabled.
/// There is no backwards compatibility guarantee for any of the types within.
///
/// [feature]: http://doc.crates.io/specifying-dependencies.html#choosing-features
#[cfg(feature = "unstable")]
pub mod unstable {
    /// An 'async' implementation of the reqwest `Client`.
    ///
    /// Relies on the `futures` crate, which is unstable, hence this module
    /// is **unstable**.
    pub mod async {
        pub use ::async_impl::{
            Body,
            Chunk,
            Decoder,
            Client,
            ClientBuilder,
            Request,
            RequestBuilder,
            Response,
        };
    }
}

/// Shortcut method to quickly make a `GET` request.
///
/// See also the methods on the [`reqwest::Response`](./struct.Response.html)
/// type.
///
/// **NOTE**: This function creates a new internal `Client` on each call,
/// and so should not be used if making many requests. Create a
/// [`Client`](./struct.Client.html) instead.
///
/// # Examples
///
/// ```rust
/// # fn run() -> Result<(), reqwest::Error> {
/// let body = reqwest::get("https://www.rust-lang.org")?
///     .text()?;
/// # Ok(())
/// # }
/// # fn main() { }
/// ```
///
/// # Errors
///
/// This function fails if:
///
/// - native TLS backend cannot be initialized
/// - supplied `Url` cannot be parsed
/// - there was an error while sending request
/// - redirect loop was detected
/// - redirect limit was exhausted
pub fn get<T: IntoUrl>(url: T) -> ::Result<Response> {
    Client::new()
        .get(url)
        .send()
}

fn _assert_impls() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    fn assert_clone<T: Clone>() {}

    assert_send::<Client>();
    assert_sync::<Client>();
    assert_clone::<Client>();

    assert_send::<Request>();
    assert_send::<RequestBuilder>();

    assert_send::<Response>();

    assert_send::<Error>();
    assert_sync::<Error>();
}

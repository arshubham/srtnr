## v0.8.4

### Features

- Add `RequestBuilder::query` to easily adjust query parameters of requests.

## v0.8.3

### Features

- Upgrades internal log crate usage to v0.4

## v0.8.2

### Fixes

- Enable hyper's `no_proto` config, fixing several bugs in hyper.

## v0.8.1

### Features

- Add `ClientBuilder::default_headers` to set headers used for every request.
- Add `async::ClientBuilder::dns_threads` to set number of threads use for DNS.
- Add `Response::text` as shortcut to read the full body into a `String`.
- Add `Response::copy_to` as shortcut for `std::io::copy`.

# v0.8.0

### Features

- Client TLS Certificates (#43)
- GZIP decoding has been added to the **async** Client (#161)
- `ClientBuilder` and `RequestBuilder` hold their errors till consumed (#189)
- `async::Response::body()` now returns a reference to the body instead of consuming the `Response`
- A default timeout for `reqwest::Client` is used set to 30 seconds (#181)

### Breaking Changes

- `Client::new` no longer returns a `Result`.

  To handle any panics that come from `Client::new`, the builder can be used instead.
- `ClientBuilder` and `RequestBuilder` hold their errors till consumed (#189).

  This means a bunch of `?` will be going away, but means using the builders will be far easier now. Any error encountered inside the builders will now be returned when the builder is consumed.

  To get errors back immediately, the `Request` type can be used directly, by building pieces separately and calling setters.
- `async::Response::body()` now returns a reference to the body instead of consuming the `Response`.
- A default timeout for `reqwest::Client` is used set to 30 seconds (#181)

  For uses where the timeout is too short, it can be changed on the `ClientBuilder`, using the `timeout` method. Passing `None` will disable the timeout, reverting to the pre-0.8 behavior.

## v0.7.3

### Features

- `Proxy::custom(fn)` to allow dynamically picking a proxy URL

### Fixes

- fix occasional panic when program exits while `Client` or `Response` are dropping.

## v0.7.2

### Fixes

- fix a panic when redirecting and a `Authorization<Basic>` header was added (https://github.com/seanmonstar/reqwest/commit/cf246d072badd9b31b487e7a0b00490e4cc9584f)
- fix redirects so that a GET will follow 307/308 responses (https://github.com/seanmonstar/reqwest/commit/2d11a4bd7167e1bf3a35b62f5aeb36d5d294e56e)

## v0.7.1

### Fixes

- fix remove accidental `println`s in the sending of a body
- some documentation improvements

# v0.7.0

### Features

- Proxy support (#30)
- Self-signed TLS certificates (#97)
- Disabling TLS hostname validation   (#89)
- A `Request` type that can be used instead of the `RequestBuilder` (#85)
- Add `Response::error_for_status()` to easily convert 400 and 500 status responses into an `Error`  (#98)
- Upgrade hyper to 0.11
  - Synchronous `Client` remains.
  - Timeouts now affect DNS and socket connection.
  - Pool much better at evicting sockets when they die.
  - An `unstable` Cargo feature to enable `reqwest::unstable::async`.
- A huge docs improvement! 

### Fixes

- Publicly exports `RedirectAction` and `RedirectAttempt`
- `Error::get_ref` returns `Error + Send + Sync`  

### Breaking Changes

- hyper has been upgraded to 0.11, so `header`, `StatusCode`, and `Method` have breaking changes.
- `mime` has been ugpraded to 0.3, with a very different API.
- All configuration methods have been removed from the `Client`, and moved to the `ClientBuilder`.
- The `HttpVersion` type was completely removed.
- `Error::cause()` now returns `Error::get_ref().cause()`.
- All methods on `Client` that start a `RequestBuilder` now return a `Result` immediately, instead of delaying the URL parse error for later.
- The `RequestBuilder` methods all take `&mut self`, instead of moving the builder, and return `&mut Self`. (This shouldn't actually affect most people who are building a request in a single chain.)
- `Response::status()` returns a `StatusCode` instead of `&StatusCode`.

## v0.6.2

### Features

- adds `Client::referer(bool)` option to disable setting the `Referer` header during redirects (https://github.com/seanmonstar/reqwest/commit/bafcd7ae6fc232856dd6ddb8bf5b20dbbbfe0bc9)

### Fixes

- fixes filtering sensitive headers during redirects (https://github.com/seanmonstar/reqwest/issues/10)
- fixes sending of the Referer to an HTTP site when coming from HTTPS, and removes username and fragment explicitly (https://github.com/seanmonstar/reqwest/commit/d8696045b4c6bc4d9e33789cff6a9e1fa75462d7)
- documentation updates

## v0.6.1

### Features

- adds `Error::get_ref` to get the underlying error that may have occurred. Includes a `'static` bounds, which allows for downcasting (as opposed to `Error::cause`).

# v0.6.0

### Features

- Upgraded to serde `1.0`
- Added a `url` [method](https://docs.rs/reqwest/0.6.0/reqwest/struct.Error.html#method.url) to `Error`, which returns a possible associated `Url` that occurred with this error.
- Added `req.basic_auth(user, optional_pass)` [method](https://docs.rs/reqwest/0.6.0/reqwest/struct.RequestBuilder.html#method.basic_auth) to ease using `Basic` authentication.

### Breaking Changes

- The publicly exposed peer dependency serde was upgraded. It is now `serde@1.0`. Mismatched version will give a compiler error that a serde trait is not implemented.
- `Error` is no longer an `enum`, but an opaque struct. Details about it can be checked with `std::error::Error::cause()`, and methods on `reqwest::Error` include `is_http()`, `is_serialization()`, and `is_redirect()`.
- `RedirectPolicy::custom` receives different arguments, and returns different values. See the [docs](https://docs.rs/reqwest/0.6.0/reqwest/struct.RedirectPolicy.html#method.custom) for an example.

## v0.5.2

### Fixes

- fix panic with Gzip decoder on an empty body (https://github.com/seanmonstar/reqwest/issues/82)

## v0.5.1

### Features

- add `Clone` implementation for `Client`

# v0.5.0

### Features

- Automatic GZIP decoding: By default, `Client` will try to decode any responses that appear to be gzip encoded (based on headers). This can be disabled via `client.gzip(false)` (https://github.com/seanmonstar/reqwest/commit/ab5e477a123319efd4b17f3666b41b44ec244bee)
- Specify a timeout for requests using `client.timeout(duration)`. (https://github.com/seanmonstar/reqwest/commit/ec049fefbae7355f6e4ddbbc7ebedcadb30e1e04)
- Request bodies with a known length can be constructed with `Body::sized()` (https://github.com/seanmonstar/reqwest/commit/82f1877d4b6cba2fac432670ec306160aee5c501)
- Add `Client.put`, `Client.patch`, and `Client.delete` convenience methods (https://github.com/seanmonstar/reqwest/commit/c37b8aa0338ac4142763d206c6df79856915056d, https://github.com/seanmonstar/reqwest/commit/4d6582d22b23c27927e481a9c8a83ad08cfd1a2a, https://github.com/seanmonstar/reqwest/commit/a3983f3122b2d1495ea36bb5a8fd019a7605ae56)
- Add `reqwest::mime` (https://github.com/seanmonstar/reqwest/commit/0615c6d65e03ba9cb5364169c9e74f4f2a91554b)

### Breaking Changes

The only breaking change is a behavioral one, all programs should still compile without modification. The automatic GZIP decoding could interfere in cases where a user was expecting the GZIP bytes, either to save to a file or decode themselves. To restore this functionality, set `client.gzip(false)`. 

# v0.4.0

- updated to serde 0.9

# v0.3.0

- updated to hyper 0.10

# v0.2.0

### Features
- add `Response.json()` method (https://github.com/seanmonstar/reqwest/commit/2d10ecc99e2aaed66616294baaf65380b446e1c6)
- add `RedirectPolicy` (https://github.com/seanmonstar/reqwest/commit/e92b3e862a1a94c0b4173a7d49a315bc121da31e)
- set an `Accept: */*` header by default if no `Accept` header is set (https://github.com/seanmonstar/reqwest/commit/559ae8011a2c098f4fe1821ec1d3444a46f4bf5e)
- add support for 307 and 308 redirects (https://github.com/seanmonstar/reqwest/commit/a54447c1d9c75dab639333265f51a91a43e99c2e)
- implement `Sync` for `Client`, and `Send` for `RequestBuilder` and `Response` (https://github.com/seanmonstar/reqwest/commit/d18a53b3fcc81c4a60875755c8e95d777a343319)
- implement `Send` for `Error` (https://github.com/seanmonstar/reqwest/commit/20b161096e67d22c962e69b2656ae9741ac73c25)
- implement `std::fmt::Debug` for all public types (https://github.com/seanmonstar/reqwest/commit/d624b0ef29020c6085ec94651a990f58ccd684e2)

### Breaking Changes
- `Error::Serialize` now has a `Box<StdError + Send + Sync>` instead of `Box<StdError>`
- `RequestBuilder` no longer has an associated lifetime (was `RequestBuilder<'a>`)

# v0.1.0

Initial release: http://seanmonstar.com/post/153221119046/introducing-reqwest

use header::Encoding;

header! {
    /// `Transfer-Encoding` header, defined in
    /// [RFC7230](http://tools.ietf.org/html/rfc7230#section-3.3.1)
    /// 
    /// The `Transfer-Encoding` header field lists the transfer coding names
    /// corresponding to the sequence of transfer codings that have been (or
    /// will be) applied to the payload body in order to form the message
    /// body.
    ///
    /// Note that setting this header will *remove* any previously set
    /// `Content-Length` header, in accordance with
    /// [RFC7230](http://tools.ietf.org/html/rfc7230#section-3.3.2):
    ///
    /// > A sender MUST NOT send a Content-Length header field in any message
    /// > that contains a Transfer-Encoding header field.
    /// 
    /// # ABNF
    ///
    /// ```text
    /// Transfer-Encoding = 1#transfer-coding
    /// ```
    /// 
    /// # Example values
    ///
    /// * `gzip, chunked`
    /// 
    /// # Example
    ///
    /// ```
    /// use hyper::header::{Headers, TransferEncoding, Encoding};
    /// 
    /// let mut headers = Headers::new();
    /// headers.set(
    ///     TransferEncoding(vec![
    ///         Encoding::Gzip,
    ///         Encoding::Chunked,
    ///     ])
    /// );
    /// ```
    (TransferEncoding, "Transfer-Encoding") => (Encoding)+

    transfer_encoding {
        test_header!(
            test1,
            vec![b"gzip, chunked"],
            Some(HeaderField(
                vec![Encoding::Gzip, Encoding::Chunked]
                )));
        // Issue: #683
        test_header!(
            test2,
            vec![b"chunked", b"chunked"],
            Some(HeaderField(
                vec![Encoding::Chunked, Encoding::Chunked]
            )));

    }
}

impl TransferEncoding {
    /// Constructor for the most common Transfer-Encoding, `chunked`.
    pub fn chunked() -> TransferEncoding {
        TransferEncoding(vec![Encoding::Chunked])
    }
}

bench_header!(normal, TransferEncoding, { vec![b"chunked, gzip".to_vec()] });
bench_header!(ext, TransferEncoding, { vec![b"ext".to_vec()] });

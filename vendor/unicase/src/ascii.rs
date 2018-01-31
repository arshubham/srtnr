#[allow(unused)]
use std::ascii::AsciiExt;
#[cfg(__unicase__iter_cmp)]
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use super::Ascii;

impl<S> Ascii<S> {
    #[inline]
    pub fn new(s: S) -> Ascii<S> {
        Ascii(s)
    }
}

impl<S> Deref for Ascii<S> {
    type Target = S;
    #[inline]
    fn deref<'a>(&'a self) -> &'a S {
        &self.0
    }
}

impl<S> DerefMut for Ascii<S> {
    #[inline]
    fn deref_mut<'a>(&'a mut self) -> &'a mut S {
        &mut self.0
    }
}

#[cfg(__unicase__iter_cmp)]
impl<T: AsRef<str>> PartialOrd for Ascii<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(__unicase__iter_cmp)]
impl<T: AsRef<str>> Ord for Ascii<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let self_chars = self.as_ref().chars().map(|c| c.to_ascii_lowercase());
        let other_chars = other.as_ref().chars().map(|c| c.to_ascii_lowercase());
        self_chars.cmp(other_chars)
    }
}

impl<S: AsRef<str>> AsRef<str> for Ascii<S> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }

}

impl<S: fmt::Display> fmt::Display for Ascii<S> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl<S1: AsRef<str>> PartialEq<Ascii<S1>> for String {
    #[inline]
    fn eq(&self, other: &Ascii<S1>) -> bool {
        other == self
    }
}

impl<'a, S1: AsRef<str>> PartialEq<Ascii<S1>> for &'a str {
    #[inline]
    fn eq(&self, other: &Ascii<S1>) -> bool {
        other == self
    }
}

impl<S1: AsRef<str>, S2: AsRef<str>> PartialEq<S2> for Ascii<S1> {
    #[inline]
    fn eq(&self, other: &S2) -> bool {
        self.as_ref().eq_ignore_ascii_case(other.as_ref())
    }
}

impl<S: AsRef<str>> Eq for Ascii<S> {}

impl<S: FromStr> FromStr for Ascii<S> {
    type Err = <S as FromStr>::Err;
    fn from_str(s: &str) -> Result<Ascii<S>, <S as FromStr>::Err> {
        s.parse().map(Ascii)
    }
}

impl<S: AsRef<str>> Hash for Ascii<S> {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        for byte in self.as_ref().bytes().map(|b| b.to_ascii_lowercase()) {
            hasher.write_u8(byte);
        }
    }
}

#[cfg(test)]
mod tests {
    use ::Ascii;
    use std::hash::{Hash, Hasher};
    #[cfg(not(__unicase__defauler_hasher))]
    use std::hash::SipHasher as DefaultHasher;
    #[cfg(__unicase__defauler_hasher)]
    use std::collections::hash_map::DefaultHasher;

    fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn test_case_insensitive() {
        let a = Ascii("foobar");
        let b = Ascii("FOOBAR");

        assert_eq!(a, b);
        assert_eq!(hash(&a), hash(&b));

        assert_eq!(a, "fooBar");
        assert_eq!("fooBar", a);
        assert_eq!(String::from("fooBar"), a);
        assert_eq!(a, String::from("fooBar"));
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_ascii_eq(b: &mut ::test::Bencher) {
        b.bytes = b"foobar".len() as u64;
        b.iter(|| assert_eq!(Ascii("foobar"), Ascii("FOOBAR")));
    }

    #[cfg(__unicase__iter_cmp)]
    #[test]
    fn test_case_cmp() {
        assert!(Ascii("foobar") == Ascii("FOOBAR"));
        assert!(Ascii("a") < Ascii("B"));

        assert!(Ascii("A") < Ascii("b"));
        assert!(Ascii("aa") > Ascii("a"));

        assert!(Ascii("a") < Ascii("aa"));
        assert!(Ascii("a") < Ascii("AA"));
    }
}

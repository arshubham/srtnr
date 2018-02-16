#![feature(test)]
extern crate test;
extern crate luhnmod10;

#[bench]
fn bench_valid(b: &mut test::Bencher) {
    b.iter(|| luhnmod10::valid("4242424242424242"));
}

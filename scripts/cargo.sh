#!/bin/sh

echo "RELEASE MODE"
cargo build --release -p srtnr && cp $1/target/release/srtnr $2


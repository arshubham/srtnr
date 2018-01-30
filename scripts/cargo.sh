#!/bin/sh

export CARGO_HOME=$1/target/cargo-home

if [[ $DEBUG = true ]]
then
    echo "DEBUG MODE"
    cargo build -p srtnr-gtk && cp $1/target/debug/srtnr-gtk $2
else
    echo "RELEASE MODE"
    cargo build --release -p srtnr-gtk && cp $1/target/release/srtnr-gtk $2
fi

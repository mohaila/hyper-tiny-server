#!/bin/sh
cargo build --target x86_64-alpine-linux-musl --release
ls -l target/x86_64-alpine-linux-musl/release/hyper-tiny-server
strip target/x86_64-alpine-linux-musl/release/hyper-tiny-server
ls -l target/x86_64-alpine-linux-musl/release/hyper-tiny-server

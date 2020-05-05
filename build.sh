#!/bin/bash

#NOTE: I dont know how to build by "sam build"
cargo build --release --target x86_64-unknown-linux-musl
zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap google_secret.json slack_secret.json
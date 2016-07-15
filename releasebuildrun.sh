#!/usr/bin/env bash
#
cargo build --release

cp target/release/wakeup /usr/local/bin/wakeup

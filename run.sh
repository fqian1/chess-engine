#!/bin/sh

case "$1" in
test | t)
    RUST_LOG=chess-engine=info cargo test -- --nocapture
    ;;
debug | d)
    RUST_BACKTRACE=1 RUST_LOG=chess-engine=info cargo run
    ;;
run | r)
    cargo run --release
    ;;
*)
    echo "Usage: $0 {test|t|debug|d|run|r}"
    ;;
esac

#!/bin/sh

COMMAND="$1"

shift

case "$COMMAND" in
test | t)
    RUST_LOG=chess-engine=info cargo test -- --nocapture "$@"
    ;;
debug | d)
    RUST_BACKTRACE=1 RUST_LOG=chess-engine=info ./target/debug/chess-engine "$@"
    ;;
run | r)
    cargo run --release "$@"
    ;;
legal_masked)
    cargo run --release -- --legal --masked -p "./legal_masked/"
    ;;
legal_unmasked)
    cargo run --release -- --legal -p "./legal_unmasked/"
    ;;
pseudo_legal_masked)
    cargo run --release -- --masked -p "./pseudo_legal_masked/"
    ;;
pseudo_legal_unmasked)
    cargo run --release -- -p "./pseudo_legal_unmasked/"
    ;;
*)
    echo "Usage: $0 {test|t|debug|d|run|r}"
    ;;
esac

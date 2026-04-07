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
legal_masked|lm)
    cargo run --release -- -l -m -p "~/chess-engine-stats/legal_masked/"
    ;;
legal_unmasked|lu)
    cargo run --release -- -l -p "~/chess-engine-stats/legal_unmasked/"
    ;;
pseudo_legal_masked|pm)
    cargo run --release -- -m -p "~/chess-engine-stats/pseudo_legal_masked/"
    ;;
pseudo_legal_unmasked|pu)
    cargo run --release -- -p "~/chess-engine-stats/pseudo_legal_unmasked/"
    ;;
*)
    echo "Usage: $0 {test|t|debug|d|run|r}"
    ;;
esac
